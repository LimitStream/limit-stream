use limit_stream::parser::*;
use std::fs;
fn main() {
  print!(">>> ");
  // let test = fs::read_to_string("./test.txt").unwrap();
  let test = "abc";
  let r = name(&test);
  println!("> {:?}", r);
}