#![feature(array_value_iter)]

mod bitfield;
mod life;
mod xorshift;

use life::Life;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
  let (w, h) = term_size::dimensions().unwrap();
  let time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;

  let mut life = Life::randomized(2 * w, 4 * h, time);
  loop {
    life.render();
    life.step();
    // thread::sleep(Duration::from_millis(10));
  }
}
