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


fn defs(i: &str) -> IResult<&str, Def> {
  alt((
    map(session_def, Def::SessionDef),
    map(struct_def, Def::StructDef),
    map(enum_def, Def::EnumDef),
  ))(i)
}

fn session_def(i: &str) -> IResult<&str, SessionDef> {
  map(tuple((tag("session"), name, tag("="), session_type)),
  |(_, name, _, session)| SessionDef{name, session})(i)
}

fn struct_def(i: &str) -> IResult<&str, StructDef> {
  map(tuple((tag("struct"), name, tag("{"), separated_list0(preceded(white_space, char(',')), struct_item), tag("}"))),
  |(_, name, _, items, _)| StructDef{name, annotation: Annotation{}, records: items})(i)
}

fn struct_item(i: &str) -> IResult<&str, (&str, (TypeOrName, Option<u64>))> {
  map(tuple((name, tag(":"), type_or_name,
    opt(preceded(tag("="), uint_lit)))), |(name, _, ty, sync)| (name, (ty, sync)))(i)
}

fn enum_def(i: &str) -> IResult<&str, EnumDef> {
  map(tuple((tag("enum"), name, tag("{"), separated_list0(preceded(white_space, char(',')), enum_item), tag("}"))),
  |(_, name, _, items, _)| EnumDef
  {name, items})(i)
}

fn enum_item(i: &str) -> IResult<&str, (&str, (TypeOrName, Option<u64>))> {
  map(tuple((name, tag("("), type_or_name, tag(")"),
    opt(preceded(tag("="), uint_lit)))), |(name, _, ty, _, sync)| (name, (ty, sync)))(i)
}

fn type_or_name(i: &str) -> IResult<&str, TypeOrName> {
  alt((
    map(_type, |t|TypeOrName::Type(Box::new(t))),
    map(name, TypeOrName::Name)))(i)
}

fn _type(i: &str) -> IResult<&str, Type> {
  alt((
    map(session_type, Type::Session),
  ))(i)
}

fn session_type(i: &str) -> IResult<&str, SessionType> {
  map(separated_list1(preceded(white_space, tag("->")), session), SessionType)(i)
}

fn session(i: &str) -> IResult<&str, Session> {
  alt((
    value(Session::Endpoint, tag("end")),
    map(preceded(tag("offer"), type_union), Session::Offer),
    map(preceded(tag("recv"), type_or_name), Session::Recv),
    map(preceded(tag("send"), type_or_name), Session::Send),
  ))(i)
}

fn type_union(i: &str) -> IResult<&str, TypeUnion> {
  map(separated_pair(type_or_name, tag("|"), type_or_name), |(a, b)| TypeUnion(a, b))(i)
}

fn name(i: &str) -> IResult<&str, &str> {
  recognize(many1(pair(not(alt((
    value((), white_space),
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

fn simple_type(i: &str) -> IResult<&str, SimpleType> {
  alt((
    value(SimpleType::Bool, tag("bool")),
    value(SimpleType::Int, tag("int")),
    value(SimpleType::Uint, tag("uint")),
    value(SimpleType::Bool, tag("bool")),
  ))(i)
}

fn constant(i: &str) -> IResult<&str, Constant> {
  alt((
    map(string_lit, Constant::String),
    map(double_lit, Constant::Float),
    map(int_lit, Constant::Int),
    map(uint_lit, Constant::Uint),
    map(bool_lit, Constant::Bool),
  ))(i)
}

fn double_lit(i: &str) -> IResult<&str, f64> {
  double(i)
}

/*
fn float_lit(i: &str) -> IResult<&str, f32> {
  float(i)
}
 */

fn int_lit(i: &str) -> IResult<&str, i64> {
  map_res(signed_number, str::parse)(i)
}

fn uint_lit(i: &str) -> IResult<&str, u64> {
  map_res(number, str::parse)(i)
}

fn signed_number(i: &str) -> IResult<&str, &str> {
  recognize(pair(alt((char('+'), char('-'))), number))(i)
}

fn number(i: &str) -> IResult<&str, &str> {
  alt((number_dec, number_hex, number_oct))(i)
}

fn number_dec(i: &str) -> IResult<&str, &str> {
  digit1(i)
}

fn number_hex(i: &str) -> IResult<&str, &str> {
  preceded(tag("0x"), hex_digit1)(i)
}

fn number_oct(i: &str) -> IResult<&str, &str> {
  preceded(tag("0o"), oct_digit1)(i)
}



fn string_lit(i: &str) -> IResult<&str, String> {
  preceded(char('"'), cut(terminated(parse_str, char('"'))))(i)
}

fn parse_str(i: &str) -> IResult<&str, String> {
  escaped_transform(anychar, '\\', alt((
    value("\\", tag("\\")),
    value("\"", tag("\"")),
    value("\n", tag("n")),
    value("\r", tag("r")),
    value("\t", tag("t")),
  )))(i)
}

fn bool_lit(i: &str) -> IResult<&str, bool> {
  alt((true_lit, false_lit))(i)
}

fn true_lit(i: &str) -> IResult<&str, bool> {
  value(true, tag("true"))(i)
}

fn false_lit(i: &str) -> IResult<&str, bool> {
  value(false, tag("false"))(i)
}

fn docu_comment(i: &str) -> IResult<&str, &str> {
  todo!()
}

fn line_comment(i: &str) -> IResult<&str, &str> {
  todo!()
}

fn white_space(i: &str) -> IResult<&str, &str> {
  let chars = " \t\r\n";
  take_while(move |c| chars.contains(c))(i)
}