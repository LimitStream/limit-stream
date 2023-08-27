use nom::character::complete::{anychar, one_of, char, alpha1, hex_digit1, oct_digit1, digit1};
use nom::{error::ParseError, IResult, bytes::complete::take_while};
use nom::bytes::complete::{tag, escaped, escaped_transform};
use nom::branch::{alt};
use nom::combinator::{map, map_res, value, cut, recognize, not};
use nom::sequence::{preceded, separated_pair, terminated, pair};
use nom::number::complete::{float, double};
use nom::multi::many1;

use crate::ast::{Constant, SimpleType};


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