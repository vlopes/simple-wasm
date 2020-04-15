use std::time::{Instant};

mod lib;
use lib::fibonacci;

fn main() {
  let start = Instant::now();
  println!("{}", fibonacci(45));
  let duration = start.elapsed();

  println!("{:?}", duration);
}