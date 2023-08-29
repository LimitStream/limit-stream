/// ```pest
/// defs = {
///   session_def |
///   struct_def |
///   enum_def
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Def<'a> {
    SessionDef(SessionDef<'a>),
    StructDef(StructDef<'a>),
    EnumDef(EnumDef<'a>),
}

/// ```pest
/// session_def = {
///  "channel" ~ name ~ "=" ~ session_type
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SessionDef<'a> {
    pub name: &'a str,
    pub session: SessionType<'a>,
}

/// ```pest
/// session_type =
///  { "end"
///  | ("offer" ~ union_type)
///  | ("choose" ~ union_type)
///  | (session_kind ~ type_or_name) ~ construct_session_type*
///  }
///
/// construct_session_type = {
///   "->" ~ session_type
/// }

/// session_kind =
///  { "recv"
///  | "send"
///  }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SessionType<'a>(pub Vec<Session<'a>>);

/// ```pest
/// struct_def = {
///   anotation ~
///   "struct" ~ name ~ "{" ~
///     (struct_item ~ ("," ~ struct_item) ~ ","?)?
///    ~ "}"
/// }
///
/// struct_item = {
///   name ~ ":" ~ type_or_name ~ ("=" ~ int_lit)?
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef<'a> {
    pub name: &'a str,
    pub annotation: Annotation,
    pub records: Vec<(&'a str, (TypeOrName<'a>, Option<u64>))>,
}

/// enum_def = {
///   "enum" ~ name ~ "{" ~
///   (enum_item ~ ("," ~ enum_item) ~ ","?)?
///    ~ "}"
/// }
///
/// enum_item = {
///   name ~ "(" ~ type_or_name ~ ")" ~ ("=" ~ int_lit)?
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef<'a> {
    pub name: &'a str,
    pub items: Vec<(&'a str, (TypeOrName<'a>, Option<u64>))>,
}
/// ```pest
/// type_or_name = { _type | name }
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TypeOrName<'a> {
    Name(&'a str),
    Type(Box<Type<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Session(SessionType<'a>),
    Struct(StructDef<'a>),
    Enum(EnumDef<'a>),
    ContainerType(ContainerType<'a>),
    SimpleType(SimpleType),
    Constant(Constant),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Session<'a> {
    Recv(TypeOrName<'a>),
    Send(TypeOrName<'a>),
    Offer(TypeUnion<'a>),
    Endpoint,
}

/// ```pest
/// union_type = {
///   type_or_name ~ "|" ~ type_or_name
/// }
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TypeUnion<'a>(pub TypeOrName<'a>, pub TypeOrName<'a>);

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerType<'a> {
    Array(Box<Type<'a>>, Option<usize>),
    Dict(SimpleType, Box<Type<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleType {
    Bool,
    Int,
    Uint,
    Float,
    Double,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    String(String),
    Float(f64),
    Int(i64),
    Uint(u64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Macro<'a, T> {
    pub appends: Vec<Append<'a>>,
    pub body: Box<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Append<'a> {
    LineComment(&'a str),
    DocsComment(&'a str),
    Annotation(Annotation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {}
