use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::ast::{
    Constant, Def, EnumDef, EnumItem, GetName, Macro, MacrodDef, Session,
    SessionDef, SessionOrName, SessionType, SessionUnion, SimpleType, StructDef, StructItem, Type,
    TypeOrName,
};

use super::Codegen;

#[derive(Debug, Clone)]
pub struct Rust {
    pub tab_size: usize,
    pub indent: usize,
    pub enum_id: Rc<Cell<usize>>,
    pub codegen_regester: Rc<RefCell<Vec<String>>>,
}

impl Rust {
    pub fn append_indent(&self) -> Self {
        Self {
            indent: self.indent + 1,
            ..self.clone()
        }
    }
    pub fn get_tab(&self) -> String {
        // TODO
        " ".repeat(self.tab_size).repeat(self.indent)
    }

    fn new_union_id(&self) -> String {
        let id = self.enum_id.as_ref().get();
        self.enum_id.as_ref().set(id + 1);
        format!("E{}", id)
    }

    fn add_to_register(&self, source: String) {
        self.codegen_regester.as_ref().borrow_mut().push(source);
    }

    pub fn anonymous_union_register(&self, union_body: &[String]) -> String {
        let items = union_body
            .iter()
            // .enumerate()
            .map(|typename| {
                format!(
                    "{}T{}({}),\n",
                    " ".repeat(self.tab_size),
                    typename,
                    typename
                )
            })
            .collect::<String>();
        let name = self.new_union_id();
        self.add_to_register(format!("pub enum {} {{\n{}}}\n", name, items));
        name
    }

    pub fn anonymous_session_register(&self, session: &str) -> String {
        let name = self.new_union_id();
        self.add_to_register(format!("pub type {} = {};\n", name, session));
        name
    }
}

impl<'a> Codegen<Rust> for MacrodDef<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        self.0.generate(generator)
    }
}

impl<'a> Codegen<Rust> for Def<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        match self {
            Def::SessionDef(d) => d.generate(generator),
            Def::StructDef(d) => d.generate(generator),
            Def::EnumDef(d) => d.generate(generator),
        }
    }
}

impl<'a> Codegen<Rust> for SessionDef<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        let session_name = self.session.generate(generator);
        format!(
            "{}pub type {} = {};\n",
            generator.get_tab(),
            self.name,
            session_name
        )
    }
}

impl<'a> Codegen<Rust> for StructDef<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        let items = self
            .items
            .iter()
            .map(|i| format!("pub {},\n", i.generate(&mut generator.append_indent())))
            .collect::<String>();
        format!(
            "{}#[rustfmt::skip]\n{}pub struct {} {{\n{}{}}}\n",
            generator.get_tab(),
            generator.get_tab(),
            self.name,
            items,
            generator.get_tab()
        )
    }
}

impl<'a> Codegen<Rust> for EnumDef<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        let items = self
            .items
            .iter()
            .map(|i| format!("{},\n", i.generate(&mut generator.append_indent())))
            .collect::<String>();
        format!(
            "{}#[rustfmt::skip]\n{}enum {} {{\n{}{}}}\n",
            generator.get_tab(),
            generator.get_tab(),
            self.name,
            items,
            generator.get_tab()
        )
    }
}

impl<'a> Codegen<Rust> for StructItem<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        format!(
            "{}{}: {}",
            generator.get_tab(),
            self.0,
            self.1.generate(generator)
        )
    }
}

impl<'a> Codegen<Rust> for EnumItem<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        if let Some(tag) = self.2 {
            format!(
                "{}{}({}) = {}",
                generator.get_tab(),
                self.0,
                self.1.generate(generator),
                tag
            )
        } else {
            format!(
                "{}{}({})",
                generator.get_tab(),
                self.0,
                self.1.generate(generator)
            )
        }
    }
}

impl<'a> Codegen<Rust> for TypeOrName<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        match self {
            TypeOrName::Name(name) => name.to_string(),
            TypeOrName::Type(ty) => ty.generate(generator),
        }
    }
}

impl<'a> Codegen<Rust> for Type<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        match self {
            Type::SessionType(session) => session.generate(generator),
            Type::ContainerType(_) => todo!(),
            Type::SimpleType(st) => st.generate(generator),
            Type::Constant(c) => c.generate(generator),
            Type::Struct(s) => s.get_name().to_string(),
            Type::Enum(e) => e.get_name().to_string(),
        }
    }
}

impl<'a> Codegen<Rust> for SessionUnion<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        let enumitem = self
            .0
            .iter()
            .map(|s| s.generate(generator))
            .collect::<Vec<String>>();
        // register anonymous session union and get name
        generator.anonymous_union_register(&enumitem)
    }
}

impl<'a> Codegen<Rust> for SessionOrName<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        match self {
            SessionOrName::Name(n) => n.to_string(),
            SessionOrName::Session(session) => {
                let session = session.generate(generator);
                // register session
                generator.anonymous_session_register(&session)
            }
        }
    }
}

impl<'a> Codegen<Rust> for SessionType<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        // todo: register anonymous session and get name
        let mut r = String::new();
        assert!(!self.0.is_empty());
        for i in self.0.iter().rev() {
            r = if r.is_empty() {
                i.generate(generator)
            } else {
                format!("Next<{}, {}>", i.generate(generator), r)
            };
        }
        r
    }
}

impl<'a> Codegen<Rust> for Session<'a> {
    fn generate(&self, generator: &mut Rust) -> String {
        match self {
            Session::Recv(ty) => format!("Recv<{}>", ty.generate(generator)),
            Session::Send(ty) => format!("Send<{}>", ty.generate(generator)),
            Session::Offer(union) => format!("Offer<{}>", union.generate(generator)),
            Session::Choose(union) => format!("Choose<{}>", union.generate(generator)),
            Session::Endpoint => "Endpoint".to_string(),
        }
    }
}

// /*
impl Codegen<Rust> for SimpleType {
    fn generate(&self, _generator: &mut Rust) -> String {
        /*
        match self {
            SimpleType::Bool => "bool",
            SimpleType::Int => "i64",
            SimpleType::Uint => "u64",
            SimpleType::Float => "f32",
            SimpleType::Double => "f64",
            SimpleType::String => "String",
        }
        .to_string()
        // */
        self.get_name().to_string()
    }
}
// */
impl Codegen<Rust> for Constant {
    fn generate(&self, _generator: &mut Rust) -> String {
        match self {
            Constant::String(s) => s.to_string(),
            Constant::Float(f) => f.to_string(),
            Constant::Int(i) => i.to_string(),
            Constant::Uint(u) => u.to_string(),
            Constant::Bool(b) => b.to_string(),
        }
    }
}

impl<'a, T: Codegen<Rust>> Codegen<Rust> for Macro<'a, T> {
    fn generate(&self, generator: &mut Rust) -> String {
        self.body.generate(generator)
    }
}

// impl<'a> Codegen<Rust> for Append<'a> {
// fn generate(&self, generator: &mut Rust) -> String {
// }
// }
// impl<'a> Codegen<Rust> for Annotation<'a> {
//     fn generate(&self, generator: &mut Rust) -> String {
//     }
// }
