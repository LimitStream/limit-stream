use limit_stream::parser::*;
// use std::fs;
fn main() {
    print!(">>> ");
    // let test = fs::read_to_string("./test.txt").unwrap();
    let test = "recv 1 -> recv string -> send 114 -> send int -> end";
    let r = _type(&test);
    println!("> {:?}", r);
}
