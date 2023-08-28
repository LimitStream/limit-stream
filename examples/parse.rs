use limit_stream::parser::*;
use std::fs;
fn main() {
  print!(">>> ");
  // let test = fs::read_to_string("./test.txt").unwrap();
  let test = "a: string = 1";
  let r = struct_item(&test);
  println!("> {:?}", r);
}