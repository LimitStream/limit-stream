use crate::ast::MacrodDef;

use super::Codegen;



struct Formatter {
    // ...
    // The current indentation level.
    indent: usize,
    // ...
}

impl<'a> Codegen<Formatter> for MacrodDef<'a> {
    fn generate(&self, generator: &mut Formatter) -> String {
        todo!()
    }
}