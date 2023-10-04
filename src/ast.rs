pub struct MacrodDef<'a>(pub Macro<'a, Def<'a>>);

pub trait GetName {
    fn get_name(&self) -> &str;
}

pub trait GetFields {
    fn get_fields(&self) -> Vec<TypeOrName>;
}

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

impl<'a> From<Def<'a>> for Type<'a> {
    fn from(value: Def<'a>) -> Self {
        match value {
            Def::SessionDef(s) => Type::SessionType(*s.session.body),
            Def::StructDef(s) => Type::Struct(s),
            Def::EnumDef(e) => Type::Enum(e),
        }
    }
}

impl<'a> GetName for Def<'a> {
    fn get_name(&self) -> &str {
        match self {
            Def::SessionDef(d) => d.get_name(),
            Def::StructDef(d) => d.get_name(),
            Def::EnumDef(d) => d.get_name(),
        }
    }
}

impl<'a> GetFields for Def<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        match self {
            Def::SessionDef(d) => d.get_fields(),
            Def::StructDef(d) => d.get_fields(),
            Def::EnumDef(d) => d.get_fields(),
        }
    }
}

/// ```pest
/// session_def = {
///   anotation ~
///   "channel" ~ name ~ "=" ~ session_type
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SessionDef<'a> {
    pub name: &'a str,
    pub session: Macro<'a, SessionType<'a>>,
}

impl<'a> GetName for SessionDef<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
}

impl<'a> GetFields for SessionDef<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        self.session.get_fields()
    }
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

impl<'a> GetFields for SessionType<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        self.0.iter().flat_map(GetFields::get_fields).collect()
    }
}

/// ```pest
/// struct_def = {
///   anotation ~
///   "struct" ~ name ~ "{" ~
///     (struct_item ~ ("," ~ struct_item) ~ ","?)?
///   ~ "}"
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef<'a> {
    pub name: &'a str,
    pub items: Vec<Macro<'a, StructItem<'a>>>,
}

impl<'a> GetName for StructDef<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
}

impl<'a> GetFields for StructDef<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        self.items.iter().flat_map(GetFields::get_fields).collect()
    }
}

/// struct_item = {
///   anotation ~ name ~ ":" ~ type_or_name ~ ("=" ~ int_lit)?
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct StructItem<'a>(pub &'a str, pub TypeOrName<'a>, pub Option<u64>);

impl<'a> GetFields for StructItem<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        vec![self.1.clone()]
    }
}

/// enum_def = {
///   anotation ~
///   "enum" ~ name ~ "{" ~
///   (enum_item ~ ("," ~ enum_item) ~ ","?)?
///    ~ "}"
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef<'a> {
    pub name: &'a str,
    pub items: Vec<Macro<'a, EnumItem<'a>>>,
}

impl<'a> GetName for EnumDef<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
}

impl<'a> GetFields for EnumDef<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        self.items.iter().flat_map(GetFields::get_fields).collect()
    }
}

/// enum_item = {
///   anotation ~ name ~ "(" ~ type_or_name ~ ")" ~ ("=" ~ int_lit)?
/// }
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem<'a>(pub &'a str, pub TypeOrName<'a>, pub Option<u64>);

impl<'a> GetFields for EnumItem<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        vec![self.1.clone()]
    }
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
    SessionType(SessionType<'a>),
    Struct(StructDef<'a>),
    Enum(EnumDef<'a>),
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

impl<'a> GetFields for SessionOrName<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        match self {
            SessionOrName::Name(_) => {
                // FIXME
                vec![]
            }
            SessionOrName::Session(s) => s.get_fields(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Session<'a> {
    Recv(TypeOrName<'a>),
    Send(TypeOrName<'a>),
    Offer(SessionUnion<'a>),
    Choose(SessionUnion<'a>),

    Endpoint,
}

impl<'a> GetFields for Session<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        match self {
            Session::Recv(r) | Session::Send(r) => vec![r.clone()],
            Session::Offer(u) | Session::Choose(u) => u.get_fields(),
            Session::Endpoint => vec![],
        }
    }
}

/// ```pest
/// session_union = {
///   session_or_name ~ "|" ~ session_or_name
/// }
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SessionUnion<'a>(pub Vec<SessionOrName<'a>>);

impl<'a> GetFields for SessionUnion<'a> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        todo!()
    }
}

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

impl<'a, T: GetFields> GetFields for Macro<'a, T> {
    fn get_fields(&self) -> Vec<TypeOrName> {
        self.body.get_fields()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Append<'a> {
    LineComment(&'a str),
    DocsComment(&'a str),
    Annotation(Annotation<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation<'a>(pub &'a str, pub Constant);
