pub mod formatter;

// pub mod rust;
// pub mod go;
// pub mod typescript;
// pub mod python;

pub trait Codegen<Generator> {
    fn generate(&self, generator: &mut Generator) -> String;
}
