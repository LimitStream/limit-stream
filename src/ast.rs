use std::collections::HashMap;



#[derive(Debug, Clone)]
pub struct SessionDef<'a> {
  name: &'a str,
  session: SessionType<'a>
}

#[derive(Debug, Clone)]
pub struct StructDef<'a> {
  name: &'a str,
  annotation: Annotation,
  records: HashMap<&'a str, (TypeOrName<'a>, usize)>,
}

#[derive(Debug, Clone)]
pub struct EnumDef<'a> {
  name: &'a str,
  items: HashMap<&'a str, (TypeOrName<'a>, usize)>,
}

#[derive(Debug, Clone)]
pub enum TypeOrName<'a> {
  Name(&'a str),
  Type(Box<Type<'a>>)
}

#[derive(Debug, Clone)]
pub enum Type<'a> {
  Session(SessionType<'a>),
  Struct(StructDef<'a>),
  Enum(EnumDef<'a>),
  ContainerType(ContainerType<'a>),
  SimpleType(SimpleType),
}

#[derive(Debug, Clone)]
pub enum SessionType<'a> {
  Recv(TypeOrName<'a>),
  Send(TypeOrName<'a>),
  Offer(TypeUnion<'a>),
}

#[derive(Debug, Clone)]
pub struct TypeUnion<'a> (pub TypeOrName<'a>, pub TypeOrName<'a>);

#[derive(Debug, Clone)]
pub enum ContainerType<'a> {
  Array(Box<Type<'a>>, Option<usize>),
  Dict(SimpleType, Box<Type<'a>>)
}

#[derive(Debug, Clone, Copy)]
pub enum SimpleType {
  Bool,
  Int,
  Uint,
  Float,
  Double,
  String,
}

#[derive(Debug, Clone)]
pub enum Constant {
  String(String),
  Float(f64),
  Int(i64),
  Uint(u64),
  Bool(bool),
}

#[derive(Debug, Clone)]
struct Annotation {

}