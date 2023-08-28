use limit_stream::parser::*;
use std::fs;
fn main() {
  print!(">>> ");
  // let test = fs::read_to_string("./test.txt").unwrap();
  let test = "struct user {
    name: string = 1,
    age: uint = 2,
    desc: string = 3
  }";
  let r = struct_def(&test);
  println!("> {:?}", r);
}