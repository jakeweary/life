pub struct Bitfield {
  size: usize,
  bytes: Vec<u8>
}

impl Bitfield {
  pub fn new(size: usize) -> Self {
    let bytes = vec![0; (size + 7) / 8];
    Self { size, bytes }
  }

  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    let size = bytes.len() * 8;
    Self { size, bytes }
  }

  pub fn at(&self, i: usize) -> bool {
    assert!(i < self.size);
    let mask = 1 << i % 8;
    *unsafe { self.bytes.get_unchecked(i / 8) } & mask != 0
  }

  pub fn flip(&mut self, i: usize) {
    assert!(i < self.size);
    let mask = 1 << i % 8;
    *unsafe { self.bytes.get_unchecked_mut(i / 8) } ^= mask;
  }

  pub fn size(&self) -> usize {
    self.size
  }
}
