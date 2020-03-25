use std::time::{Instant};

mod lib;
use lib::fibonacci;

fn main() {
  let start = Instant::now();
  println!("Fibonacci generator");
  println!("{}", fibonacci(1));
  println!("{}", fibonacci(3));
  println!("{}", fibonacci(30));
  let duration = start.elapsed();

  println!("Finished in {:?}", duration);
}