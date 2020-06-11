use std::iter::{FromIterator, IntoIterator};

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

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn get(&self, i: usize) -> bool {
    let (byte, shift) = self.pair(i);
    *byte & (1 << shift) != 0
  }

  pub fn set(&mut self, i: usize, v: bool) {
    let (byte, shift) = self.pair_mut(i);
    *byte &= !(1 << shift);
    *byte |= (v as u8) << shift;
  }

  pub fn off(&mut self, i: usize) {
    let (byte, shift) = self.pair_mut(i);
    *byte &= !(1 << shift);
  }

  pub fn on(&mut self, i: usize) {
    let (byte, shift) = self.pair_mut(i);
    *byte |= 1 << shift;
  }

  pub fn flip(&mut self, i: usize) {
    let (byte, shift) = self.pair_mut(i);
    *byte ^= 1 << shift;
  }

  fn pair(&self, i: usize) -> (&u8, usize) {
    debug_assert!(i < self.size);
    let byte = unsafe { self.bytes.get_unchecked(i / 8) };
    let shift = i % 8;
    (byte, shift)
  }

  fn pair_mut(&mut self, i: usize) -> (&mut u8, usize) {
    debug_assert!(i < self.size);
    let byte = unsafe { self.bytes.get_unchecked_mut(i / 8) };
    let shift = i % 8;
    (byte, shift)
  }
}

impl FromIterator<u8> for Bitfield {
  fn from_iter<I>(iter: I) -> Self
  where I: IntoIterator<Item = u8> {
    let bytes = iter.into_iter().collect();
    Bitfield::from_bytes(bytes)
  }
}
