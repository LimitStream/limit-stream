use limit_stream::codegen::formatter::Formatter;
use limit_stream::codegen::{formatter, Codegen};
use limit_stream::parser::*;

fn main() {
    let (_, ast) = macrod_def(
        "channel shit = recv int -> send int -> recv int -> offer sum | recv int -> end",
    )
    .unwrap();
    let src = ast.generate(&mut Formatter {
        tab_size: 4,
        indent: 0,
    });
    println!("{}", src);
}
