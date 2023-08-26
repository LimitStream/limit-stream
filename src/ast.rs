use std::collections::HashMap;



struct SessionDef<'a> {
  name: &'a str,
  session: SessionType<'a>
}

struct StructDef<'a> {
  name: &'a str,
  annotation: Annotation,
  records: HashMap<&'a str, (TypeOrName<'a>, usize)>,
}

struct EnumDef<'a> {
  name: &'a str,
  items: HashMap<&'a str, (TypeOrName<'a>, usize)>,
}


enum TypeOrName<'a> {
  Name(&'a str),
  Type(Box<Type<'a>>)
}

enum Type<'a> {
  Session(SessionType<'a>),
  Struct(StructDef<'a>),
  Enum(EnumDef<'a>),
  ContainerType(ContainerType<'a>),
  SimpleType(SimpleType),
}


enum SessionType<'a> {
  Recv(TypeOrName<'a>),
  Send(TypeOrName<'a>),
  Offer(TypeUnion<'a>),
}

struct TypeUnion<'a> (pub TypeOrName<'a>, pub TypeOrName<'a>);

enum ContainerType<'a> {
  Array(Box<Type<'a>>, Option<usize>),
  Dict(SimpleType, Box<Type<'a>>)
}

enum SimpleType {
  Bool,
  Int,
  Uint,
  Float,
  Double,
  String,
}

struct Annotation {

}