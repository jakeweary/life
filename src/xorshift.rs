pub struct Xorshift {
  state: u64
}

impl Xorshift {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }

  pub fn next_u64(&mut self) -> u64 {
    self.state = next(self.state);
    self.state
  }

  pub fn next_f64(&mut self) -> f64 {
    norm(self.next_u64())
  }
}

impl Iterator for Xorshift {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.next_u64())
  }
}

pub fn next(mut n: u64) -> u64 {
  n ^= n << 13;
  n ^= n >> 7;
  n ^= n << 17;
  n
}

pub fn norm(mut n: u64) -> f64 {
  n = 0x3ff<<52 | n>>12;
  f64::from_bits(n) - 1.0
}
