use std::io::{stdin, stdout, Write};

use limit_stream::parser::*;

fn main() {
  loop {
  let mut source = String::new();
  print!(">>> ");
  stdout().flush().unwrap();
  stdin().read_line(&mut source).unwrap();
  // let mut source = "<type your example>".to_string();
  let r = _type(&source);
  println!("> {:?}", r);
  }
}