pub struct Xorshift {
  state: u64
}

impl Xorshift {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }

  pub fn next_u64(&mut self) -> u64 {
    self.state ^= self.state << 13;
    self.state ^= self.state >> 7;
    self.state ^= self.state << 17;
    self.state
  }

  pub fn next_f64(&mut self) -> f64 {
    let bits = 0x3ff<<52 | self.next_u64()>>12;
    f64::from_bits(bits) - 1.0
  }
}
