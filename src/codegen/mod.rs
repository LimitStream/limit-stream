use crate::parser::parse;

use self::{formatter::Formatter, rust::Rust};

pub mod formatter;

pub mod rust;
// pub mod go;
// pub mod typescript;
// pub mod python;

pub trait Codegen<Generator> {
    fn generate(&self, generator: &mut Generator) -> String;
}

pub fn format_idl(src: &str, rs: &mut Formatter) -> String {
    let asts = parse(&src).unwrap();
    asts.into_iter()
        .map(|ast| ast.generate(rs))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn idl2rust(src: &str, rs: &mut Rust) -> String {
    let asts = parse(&src).unwrap();
    let code_body = asts
        .into_iter()
        .map(|ast| ast.generate(rs))
        .collect::<Vec<_>>()
        .join("\n");
    let mut code = rs.codegen_regester.as_ref().borrow().join("\n");
    code.push_str("\n");
    code.push_str(&code_body);
    code
}
