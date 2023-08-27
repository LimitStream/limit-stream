use std::collections::HashMap;

use nom::character::complete::{anychar, one_of, char, alpha1, hex_digit1, oct_digit1, digit1};
use nom::{error::ParseError, IResult, bytes::complete::take_while};
use nom::bytes::complete::{tag, escaped, escaped_transform};
use nom::branch::{alt};
use nom::combinator::{map, map_res, value, cut, recognize, not, opt};
use nom::sequence::{preceded, separated_pair, terminated, pair, tuple};
use nom::number::complete::{float, double};
use nom::multi::{many1, many0, separated_list1, separated_list0};

use crate::ast::{Constant, SimpleType, StructDef, SessionType, TypeUnion, Type, TypeOrName, Session, SessionDef, Def, Annotation, EnumDef};


pub fn def(i: &str) -> IResult<&str, Def> {
  alt((
    map(session_def, Def::SessionDef),
    map(struct_def, Def::StructDef),
    map(enum_def, Def::EnumDef),
  ))(i)
}

pub fn session_def(i: &str) -> IResult<&str, SessionDef> {
  map(tuple((
    preceded(ws, tag("session")),
    preceded(ws, name),
    preceded(ws, tag("=")),
    preceded(ws, session_type))),
  |(_, name, _, session)| SessionDef{name, session})(i)
}

pub fn struct_def(i: &str) -> IResult<&str, StructDef> {
  map(tuple((
    preceded(ws, tag("struct")),
    preceded(ws, name),
    preceded(ws, tag("{")),
    preceded(ws, separated_list0(preceded(ws, char(',')), preceded(ws, struct_item))),
    preceded(ws, tag("}")))),
  |(_, name, _, items, _)| StructDef{name, annotation: Annotation{}, records: items})(i)
}

pub fn struct_item(i: &str) -> IResult<&str, (&str, (TypeOrName, Option<u64>))> {
  map(tuple((
    preceded(ws, name),
    preceded(ws, tag(":")),
    preceded(ws, type_or_name),
    preceded(ws, opt(preceded(tag("="), uint_lit))
  ))), |(name, _, ty, sync)| (name, (ty, sync)))(i)
}

pub fn enum_def(i: &str) -> IResult<&str, EnumDef> {
  map(tuple((
    preceded(ws, tag("enum")),
    preceded(ws, name),
    preceded(ws, tag("{")),
    preceded(ws, separated_list0(preceded(ws, char(',')), preceded(ws, enum_item))),
    preceded(ws, tag("}"))
  )),
  |(_, name, _, items, _)| EnumDef
  {name, items})(i)
}

pub fn enum_item(i: &str) -> IResult<&str, (&str, (TypeOrName, Option<u64>))> {
  map(tuple((
    preceded(ws, name),
    preceded(ws, tag("(")),
    preceded(ws, type_or_name),
    preceded(ws, tag(")")),
    preceded(ws, opt(preceded(tag("="), uint_lit)))
  )), |(name, _, ty, _, sync)| (name, (ty, sync)))(i)
}

pub fn type_or_name(i: &str) -> IResult<&str, TypeOrName> {
  alt((
    map(_type, |t|TypeOrName::Type(Box::new(t))),
    map(name, TypeOrName::Name)))(i)
}

pub fn _type(i: &str) -> IResult<&str, Type> {
  alt((
    map(session_type, Type::Session),
    map(simple_type, Type::SimpleType),
    map(constant, Type::Constant)
  ))(i)
}

pub fn session_type(i: &str) -> IResult<&str, SessionType> {
  map(separated_list1(preceded(ws, tag("->")), preceded(ws, session)), SessionType)(i)
}

pub fn session(i: &str) -> IResult<&str, Session> {
  alt((
    value(Session::Endpoint, tag("end")),
    map(preceded(ws, preceded(tag("offer"), preceded(ws, type_union))), Session::Offer),
    map(preceded(ws, preceded(tag("recv"), preceded(ws, type_or_name))), Session::Recv),
    map(preceded(ws, preceded(tag("send"), preceded(ws, type_or_name))), Session::Send),
  ))(i)
}

pub fn type_union(i: &str) -> IResult<&str, TypeUnion> {
  map(separated_pair(
    preceded(ws, type_or_name),
    preceded(ws, tag("|")),
    preceded(ws, type_or_name)
  ), |(a, b)| TypeUnion(a, b))(i)
}

pub fn name(i: &str) -> IResult<&str, &str> {
  recognize(many1(pair(not(alt((
    value((), ws),
    value((), simple_type),
    value((), alt((
      tag("end"),
      tag("recv"),
      tag("send"),
      tag("offer"),
      tag("session"),
      tag("struct"),
      tag("enum"),
      tag("->"),
      tag("#"),
      tag("["),
      tag("]"),
      tag("{"),
      tag("}"),
      tag(","),
      tag("="),
      tag("|"),
    ))),
    value((), constant),
  ))), anychar)))(i)
}

pub fn simple_type(i: &str) -> IResult<&str, SimpleType> {
  alt((
    value(SimpleType::Bool, tag("bool")),
    value(SimpleType::Int, tag("int")),
    value(SimpleType::Uint, tag("uint")),
    value(SimpleType::Float, tag("float")),
    value(SimpleType::Double, tag("double")),
    value(SimpleType::String, tag("string")),
  ))(i)
}

pub fn constant(i: &str) -> IResult<&str, Constant> {
  alt((
    map(string_lit, Constant::String),
    map(double_lit, Constant::Float),
    map(int_lit, Constant::Int),
    map(uint_lit, Constant::Uint),
    map(bool_lit, Constant::Bool),
  ))(i)
}

pub fn double_lit(i: &str) -> IResult<&str, f64> {
  double(i)
}

/*
pub fn float_lit(i: &str) -> IResult<&str, f32> {
  float(i)
}
 */

pub fn int_lit(i: &str) -> IResult<&str, i64> {
  map_res(signed_number, str::parse)(i)
}

pub fn uint_lit(i: &str) -> IResult<&str, u64> {
  map_res(number, str::parse)(i)
}

pub fn signed_number(i: &str) -> IResult<&str, &str> {
  recognize(pair(alt((char('+'), char('-'))), number))(i)
}

pub fn number(i: &str) -> IResult<&str, &str> {
  alt((number_dec, number_hex, number_oct))(i)
}

pub fn number_dec(i: &str) -> IResult<&str, &str> {
  digit1(i)
}

pub fn number_hex(i: &str) -> IResult<&str, &str> {
  preceded(tag("0x"), hex_digit1)(i)
}

pub fn number_oct(i: &str) -> IResult<&str, &str> {
  preceded(tag("0o"), oct_digit1)(i)
}

pub fn string_lit(i: &str) -> IResult<&str, String> {
  preceded(char('"'), cut(terminated(parse_str, char('"'))))(i)
}

pub fn parse_str(i: &str) -> IResult<&str, String> {
  escaped_transform(anychar, '\\', alt((
    value("\\", tag("\\")),
    value("\"", tag("\"")),
    value("\n", tag("n")),
    value("\r", tag("r")),
    value("\t", tag("t")),
  )))(i)
}

pub fn bool_lit(i: &str) -> IResult<&str, bool> {
  alt((true_lit, false_lit))(i)
}

pub fn true_lit(i: &str) -> IResult<&str, bool> {
  value(true, tag("true"))(i)
}

pub fn false_lit(i: &str) -> IResult<&str, bool> {
  value(false, tag("false"))(i)
}

pub fn docu_comment(i: &str) -> IResult<&str, &str> {
  todo!()
}

pub fn line_comment(i: &str) -> IResult<&str, &str> {
  todo!()
}

pub fn ws(i: &str) -> IResult<&str, &str> {
  let chars = " \t\r\n";
  take_while(move |c| chars.contains(c))(i)
}