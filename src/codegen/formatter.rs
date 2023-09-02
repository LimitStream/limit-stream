use crate::ast::{
    Annotation, Append, Constant, Def, EnumDef, EnumItem, Macro, MacrodDef, Session, SessionDef,
    SessionOrName, SessionType, SessionUnion, SimpleType, StructDef, StructItem, Type, TypeOrName,
};

use super::Codegen;

#[derive(Debug, Clone)]
pub struct Formatter {
    // ...
    // The current indentation level.
    pub tab_size: usize,
    pub indent: usize,
    // ...
}

impl Formatter {
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
}

impl<'a> Codegen<Formatter> for MacrodDef<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        self.0.generate(generator)
    }
}

impl<'a> Codegen<Formatter> for Def<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            Def::SessionDef(d) => d.generate(generator),
            Def::StructDef(d) => d.generate(generator),
            Def::EnumDef(d) => d.generate(generator),
        }
    }
}

impl<'a> Codegen<Formatter> for SessionDef<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        let session = self.session.generate(&mut generator.append_indent());
        format!(
            "{}session {} = \n{}{}",
            generator.get_tab(),
            self.name,
            generator.get_tab(),
            session
        )
    }
}

impl<'a> Codegen<Formatter> for StructDef<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        let items = self
            .items
            .iter()
            .map(|i| format!("{},\n", i.generate(&mut generator.append_indent())))
            .collect::<String>();
        format!(
            "{}struct {} {{\n{}{}}}",
            generator.get_tab(),
            self.name,
            items,
            generator.get_tab()
        )
    }
}

impl<'a> Codegen<Formatter> for EnumDef<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        let items = self
            .items
            .iter()
            .map(|i| format!("{},\n", i.generate(&mut generator.append_indent())))
            .collect::<String>();
        format!(
            "{}enum {} {{\n{}{}}}",
            generator.get_tab(),
            self.name,
            items,
            generator.get_tab()
        )
    }
}

impl<'a> Codegen<Formatter> for StructItem<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        format!(
            "{}{}: {}{}",
            generator.get_tab(),
            self.0,
            self.1.generate(generator),
            if let Some(s) = self.2 {
                format!(" = {}", s.to_string())
            } else {
                "".to_string()
            }
        )
    }
}

impl<'a> Codegen<Formatter> for EnumItem<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        format!(
            "{}{}({}){}",
            generator.get_tab(),
            self.0,
            self.1.generate(generator),
            if let Some(s) = self.2 {
                format!(" = {}", s.to_string())
            } else {
                "".to_string()
            }
        )
    }
}

impl<'a> Codegen<Formatter> for TypeOrName<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            TypeOrName::Name(name) => name.to_string(),
            TypeOrName::Type(ty) => ty.generate(generator),
        }
    }
}

impl<'a> Codegen<Formatter> for Type<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            Type::SessionType(session) => session.generate(generator),
            Type::ContainerType(_) => todo!(),
            Type::SimpleType(st) => st.generate(generator),
            Type::Constant(c) => c.generate(generator),
        }
    }
}

impl<'a> Codegen<Formatter> for SessionUnion<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        generator.indent += 1;
        let mut res = String::new();
        for s in self.0.iter() {
            res += &format!(
                "\n{}| {}",
                generator.get_tab(),
                &s.generate(generator)[generator.indent * generator.tab_size..]
                    .replace("\n", "\n  ")
            );
        }
        generator.indent -= 1;
        res
    }
}

impl<'a> Codegen<Formatter> for SessionOrName<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            SessionOrName::Name(name) => format!("{}{}", generator.get_tab(), name),
            SessionOrName::Session(session) => session.generate(generator),
        }
    }
}

impl<'a> Codegen<Formatter> for SessionType<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        self.0
            .iter()
            .map(|m| format!("{}{}", generator.get_tab(), m.generate(generator)))
            .collect::<Vec<String>>()
            .join(" ->\n") // FIXME
    }
}

impl<'a> Codegen<Formatter> for Session<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            Session::Recv(ty) => format!("recv {}", ty.generate(generator)),
            Session::Send(ty) => format!("send {}", ty.generate(generator)),
            Session::Offer(union) => format!("offer {}", union.generate(generator)),
            Session::Choose(union) => format!("choose {}", union.generate(generator)),
            Session::Endpoint => "end".to_string(),
        }
    }
}

impl Codegen<Formatter> for SimpleType {
    fn generate(&self, _generator: &mut Formatter) -> String {
        match self {
            SimpleType::Bool => "bool",
            SimpleType::Int => "int",
            SimpleType::Uint => "uint",
            SimpleType::Float => "float",
            SimpleType::Double => "double",
            SimpleType::String => "string",
        }
        .to_string()
    }
}

impl Codegen<Formatter> for Constant {
    fn generate(&self, _generator: &mut Formatter) -> String {
        match self {
            Constant::String(s) => s.to_string(),
            Constant::Float(f) => f.to_string(),
            Constant::Int(i) => i.to_string(),
            Constant::Uint(u) => u.to_string(),
            Constant::Bool(b) => b.to_string(),
        }
        .to_string()
    }
}

impl<'a, T: Codegen<Formatter>> Codegen<Formatter> for Macro<'a, T> {
    fn generate(&self, generator: &mut Formatter) -> String {
        let append = self
            .appends
            .iter()
            .map(|f| format!("{}{}\n", generator.get_tab(), f.generate(generator)))
            .collect::<String>();
        let body = self.body.generate(generator);
        format!("{}{}", append, body)
    }
}

impl<'a> Codegen<Formatter> for Append<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        match self {
            Append::LineComment(s) | Append::DocsComment(s) => s.to_string(),
            Append::Annotation(a) => a.generate(generator),
        }
    }
}

impl<'a> Codegen<Formatter> for Annotation {
    fn generate(&self, _generator: &mut Formatter) -> String {
        format!("todo")
    }
}
