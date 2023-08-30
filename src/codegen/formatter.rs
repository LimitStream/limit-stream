use crate::ast::MacrodDef;

use super::Codegen;



struct Formatter {
    // ...
    // The current indentation level.
    indent: usize,
    // ...
}

impl Codegen<Formatter> for MacrodDef {
    fn generate(&self, generator: &mut Formatter) -> String {
        todo!()
    }
}