use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_a, tag};
use nom::character::complete::{anychar, char, digit1, hex_digit1, oct_digit1};
use nom::combinator::{cut, map, map_res, not, opt, recognize, value};

use nom::multi::{many0, many1, many_m_n, separated_list0, separated_list1};

use nom::sequence::{pair, preceded, terminated, tuple};
use nom::{bytes::complete::take_while, IResult};

use crate::ast::{
    Annotation, Append, Constant, Def, EnumDef, EnumItem, Macro, MacrodDef, Session, SessionDef,
    SessionOrName, SessionType, SessionUnion, SimpleType, StructDef, StructItem, Type, TypeOrName,
};

/*
#[macro_export]
macro_rules! macro_gen {
    ($f: expr) => {
        |i: & str| {
            let (i, appends) = many0(preceded(ws, append))(i)?;
            map($f, move |r| Macro {
                appends: appends.clone(),
                body: Box::new(r),
            })(i)
        }
    };
}
// */

pub fn parse(i: &str) -> Result<Vec<MacrodDef>, String> {
    let (str, r) = many1(macrod_def)(i.trim()).map_err(|e| format!("{}", e))?;
    if !str.is_empty() {
        return Err("parse failed to end".to_string());
    }
    Ok(r)
}

pub fn macrod_def(i: &str) -> IResult<&str, MacrodDef> {
    preceded(ws, map(_macro(preceded(ws, def)), MacrodDef))(i)
}

pub fn def(i: &str) -> IResult<&str, Def> {
    alt((
        map(session_def, Def::SessionDef),
        map(struct_def, Def::StructDef),
        map(enum_def, Def::EnumDef),
    ))(i)
}

pub fn session_def(i: &str) -> IResult<&str, SessionDef> {
    map(
        tuple((
            preceded(ws, tag("channel")),
            preceded(ws, name),
            preceded(ws, tag("=")),
            preceded(ws, _macro(preceded(ws, session_type))),
        )),
        |(_, name, _, session)| SessionDef { name, session },
    )(i)
}

pub fn struct_def(i: &str) -> IResult<&str, StructDef> {
    map(
        tuple((
            preceded(ws, tag("struct")),
            preceded(ws, name),
            preceded(ws, tag("{")),
            preceded(
                ws,
                terminated(
                    separated_list0(
                        preceded(ws, char(',')),
                        preceded(ws, _macro(preceded(ws, struct_item))),
                    ),
                    opt(char(',')),
                ),
            ),
            preceded(ws, tag("}")),
        )),
        |(_, name, _, items, _)| StructDef { name, items },
    )(i)
}

pub fn enum_def(i: &str) -> IResult<&str, EnumDef> {
    map(
        tuple((
            preceded(ws, tag("enum")),
            preceded(ws, name),
            preceded(ws, tag("{")),
            preceded(
                ws,
                terminated(
                    separated_list0(
                        preceded(ws, char(',')),
                        preceded(ws, _macro(preceded(ws, enum_item))),
                    ),
                    opt(char(',')),
                ),
            ),
            preceded(ws, tag("}")),
        )),
        |(_, name, _, items, _)| EnumDef { name, items },
    )(i)
}

pub fn struct_item(i: &str) -> IResult<&str, StructItem> {
    map(
        tuple((
            preceded(ws, name),
            preceded(ws, tag(":")),
            preceded(ws, type_or_name),
            opt(preceded(preceded(ws, tag("=")), preceded(ws, uint_lit))),
        )),
        |(name, _, ty, sync)| StructItem(name, ty, sync),
    )(i)
}

pub fn enum_item(i: &str) -> IResult<&str, EnumItem> {
    map(
        tuple((
            preceded(ws, name),
            preceded(ws, tag("(")),
            preceded(ws, type_or_name),
            preceded(ws, tag(")")),
            opt(preceded(preceded(ws, tag("=")), preceded(ws, uint_lit))),
        )),
        |(name, _, ty, _, sync)| EnumItem(name, ty, sync),
    )(i)
}

pub fn type_or_name(i: &str) -> IResult<&str, TypeOrName> {
    alt((
        map(_type, |t| TypeOrName::Type(Box::new(t))),
        map(name, TypeOrName::Name),
    ))(i)
}

pub fn _type(i: &str) -> IResult<&str, Type> {
    alt((
        map(session_type, Type::SessionType),
        map(simple_type, Type::SimpleType),
        map(constant, Type::Constant),
    ))(i)
}

pub fn session_or_name(i: &str) -> IResult<&str, SessionOrName> {
    alt((
        map(session_type, |t| SessionOrName::Session(Box::new(t))),
        map(name, SessionOrName::Name),
    ))(i)
}

pub fn session_type(i: &str) -> IResult<&str, SessionType> {
    map(
        separated_list1(
            preceded(ws, tag("->")),
            preceded(ws, _macro(preceded(ws, session))),
        ),
        SessionType,
    )(i)
}

pub fn session(i: &str) -> IResult<&str, Session> {
    alt((
        value(Session::Endpoint, tag("end")),
        map(
            preceded(ws, preceded(tag("offer"), preceded(ws, session_union))),
            Session::Offer,
        ),
        map(
            preceded(ws, preceded(tag("choose"), preceded(ws, session_union))),
            Session::Choose,
        ),
        map(
            preceded(ws, preceded(tag("recv"), preceded(ws, type_or_name))),
            Session::Recv,
        ),
        map(
            preceded(ws, preceded(tag("send"), preceded(ws, type_or_name))),
            Session::Send,
        ),
    ))(i)
}

pub fn session_union(i: &str) -> IResult<&str, SessionUnion> {
    map(
        many_m_n(
            2,
            10,
            preceded(ws, preceded(tag("|"), preceded(ws, session_or_name))),
        ),
        SessionUnion,
    )(i)
    // map(
    //     separated_pair(
    //         preceded(ws, session_or_name),
    //         preceded(ws, tag("|")),
    //         preceded(ws, session_or_name),
    //     ),
    //     |(a, b)| SessionUnion(a, b),
    // )(i)
}

pub fn name(i: &str) -> IResult<&str, &str> {
    recognize(many1(pair(
        not(alt((
            value((), simple_type),
            value((), is_a(" \t\r\n")), // white space
            value((), tag("end")),
            value((), tag("recv")),
            value((), tag("send")),
            value((), tag("offer")),
            value((), tag("choose")),
            value((), tag("channel")),
            value((), tag("struct")),
            value((), tag("enum")),
            value(
                (),
                alt((
                    tag("->"),
                    tag("#"),
                    tag("["),
                    tag("]"),
                    tag("{"),
                    tag("}"),
                    tag("("),
                    tag(")"),
                    tag(","),
                    tag(":"),
                    tag("="),
                    tag("|"),
                )),
            ),
            value((), constant),
        ))),
        anychar,
    )))(i)
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
    map_res(
        recognize(tuple((signed_number, tag("."), number))),
        str::parse,
    )(i)
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
    escaped_transform(
        anychar,
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
            value("\r", tag("r")),
            value("\t", tag("t")),
        )),
    )(i)
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

// /*
pub fn _macro<
    'i,
    // 'i1: 'i,
    'r: 'i,
    R: Clone,
    // I: FnMut(&str) -> IResult<&str, R>,
    I: FnMut(&'i str) -> IResult<&'i str, R>,
    // G: Fn(&'a str) -> IResult<&'a str, Macro<'a, R>>,
>(
    // f: impl FnMut(&'a str) -> IResult<&'a str, R>,
    // ) -> impl FnMut(&'a str) -> IResult<&'a str, Macro<'a, R>> {
    mut f: I,
    // ) -> impl FnMut(&str) -> IResult<&str, Macro<R>> {
) -> impl FnMut(&'i str) -> IResult<&'i str, Macro<'i, R>> {
    move |i: &'i str| {
        let (i, appends): (&'i str, Vec<Append<'i>>) = many0(preceded(ws, append))(i)?;
        let (i, r): (&'i str, R) = f(i)?;
        Ok((
            i,
            Macro {
                appends,
                body: Box::new(r),
            },
        ))
    }
}
//  */
pub fn append(i: &str) -> IResult<&str, Append> {
    alt((
        map(docu_comment, Append::DocsComment),
        map(line_comment, Append::LineComment),
        map(annotation, Append::Annotation),
    ))(i)
}

pub fn annotation(i: &str) -> IResult<&str, Annotation> {
    map(
        tuple((
            preceded(ws, tag("#")),
            preceded(ws, tag("[")),
            preceded(ws, annotation_body),
            preceded(ws, tag("]")),
        )),
        |(_, _, a, _)| a,
    )(i)
}

pub fn annotation_body(i: &str) -> IResult<&str, Annotation> {
    alt((
        map(
            tuple((
                preceded(ws, name),
                preceded(ws, tag("=")),
                preceded(ws, constant),
            )),
            |(name, _, c)| Annotation(name, c),
        ),
        map(preceded(ws, name), |name| {
            Annotation(name, Constant::Bool(true))
        }),
    ))(i)
}

pub fn docu_comment(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        preceded(ws, tag("///")),
        many0(recognize(tuple((not(tag("\n")), anychar)))),
        opt(tag("\n")),
    )))(i)
}

pub fn line_comment(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        preceded(ws, tag("//")),
        many0(recognize(tuple((not(tag("\n")), anychar)))),
        opt(tag("\n")),
    )))(i)
}

pub fn ws(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
