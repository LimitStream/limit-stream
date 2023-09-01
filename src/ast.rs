pub struct MacrodDef<'a>(pub Macro<'a, Def<'a>>);

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
    pub session: Macro<'a, SessionType<'a>>,
}

/// ```pest
/// session_type =
///  { "end"
///  | ("offer" ~ session_union)
///  | ("choose" ~ session_union)
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
pub struct SessionType<'a>(pub Vec<Macro<'a, Session<'a>>>);

/// ```pest
/// struct_def = {
///   anotation ~
///   "struct" ~ name ~ "{" ~
///     (struct_item ~ ("," ~ struct_item) ~ ","?)?
///    ~ "}"
/// }
///

/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef<'a> {
    pub name: &'a str,
    pub items: Vec<Macro<'a, StructItem<'a>>>,
}

/// struct_item = {
///   name ~ ":" ~ type_or_name ~ ("=" ~ int_lit)?
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct StructItem<'a>(pub &'a str, pub TypeOrName<'a>, pub Option<u64>);

/// enum_def = {
///   "enum" ~ name ~ "{" ~
///   (enum_item ~ ("," ~ enum_item) ~ ","?)?
///    ~ "}"
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef<'a> {
    pub name: &'a str,
    pub items: Vec<Macro<'a, EnumItem<'a>>>,
}

/// enum_item = {
///   name ~ "(" ~ type_or_name ~ ")" ~ ("=" ~ int_lit)?
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem<'a>(pub &'a str, pub TypeOrName<'a>, pub Option<u64>);

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
    SessionType(SessionType<'a>),
    // Struct(StructDef<'a>),
    // Enum(EnumDef<'a>),
    ContainerType(ContainerType<'a>), // todo
    SimpleType(SimpleType),
    Constant(Constant),
}

/// ```pest
/// session_or_name = { session | name }
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum SessionOrName<'a> {
    Name(&'a str),
    Session(Box<SessionType<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Session<'a> {
    Recv(TypeOrName<'a>),
    Send(TypeOrName<'a>),
    Offer(SessionUnion<'a>),
    Choose(SessionUnion<'a>),

    Endpoint,
}

/// ```pest
/// session_union = {
///   session_or_name ~ "|" ~ session_or_name
/// }
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SessionUnion<'a>(pub Vec<SessionOrName<'a>>);

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
