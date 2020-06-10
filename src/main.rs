#![feature(array_value_iter)]
#![allow(dead_code)]

mod bitfield;
mod life;
mod xorshift;

use life::Life;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
  let (w, h) = term_size::dimensions().unwrap();
  let time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;

  let mut life = Life::randomized(2 * w, 4 * h, time);
  loop {
    print!("\x1b[2J\x1b[1;1H{}", life.render());
    io::stdout().flush().unwrap();
    life.step();
  }
}
