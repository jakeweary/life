use super::bitfield::Bitfield;
use super::cell::Cell;
use super::xorshift::xorshift;
use std::array::IntoIter;
use std::io::{self, Write};
use std::iter;

const RULES: [u16; 2] = [
  // 876543210 neighbors alive
  0b_000001000, // dead
  0b_000001100  // alive
];

const NEIGHBORS: [(usize, usize); 8] = [
  (0, 0), (1, 0), (2, 0),
  (0, 1),         (2, 1),
  (0, 2), (1, 2), (2, 2)
];

const BRAILLE: [((usize, usize), u8); 8] = [
  ((0, 0), 1 << 0), ((1, 0), 1 << 3), // ●₀ ●₃
  ((0, 1), 1 << 1), ((1, 1), 1 << 4), // ●₁ ●₄
  ((0, 2), 1 << 2), ((1, 2), 1 << 5), // ●₂ ●₅
  ((0, 3), 1 << 6), ((1, 3), 1 << 7)  // ○₆ ○₇
];

pub struct Life {
  width: usize,
  height: usize,
  cells: Bitfield
}

impl Life {
  pub fn new(width: usize, height: usize, seed: u64) -> Self {
    let mut next = xorshift(seed);

    let bytes = iter::from_fn(move || Some(next()))
      .flat_map(|n| IntoIter::new(n.to_ne_bytes()))
      .take(width * height >> 3)
      .collect();

    let cells = Bitfield::from_bytes(bytes);
    Self { width, height, cells }
  }

  pub fn to_index(&self, x: usize, y: usize) -> usize {
    let x = (x + self.width) % self.width;
    let y = (y + self.height) % self.height;
    self.width * y + x
  }

  pub fn at(&self, x: usize, y: usize) -> Cell {
    Cell::from_bool(self.cells.at(self.to_index(x, y)))
  }

  pub fn neighbors(&self, x: usize, y: usize) -> usize {
    NEIGHBORS.iter()
      .filter(|(u, v)| self.at(x + u - 1, y + v - 1).is_alive())
      .map(|_| 1)
      .sum()
  }

  pub fn step(&mut self) {
    let mut cells = Bitfield::new(self.cells.size());

    for y in 0..self.height {
      for x in 0..self.width {
        let c = self.at(x, y);
        let n = self.neighbors(x, y);
        if RULES[c as usize] >> n & 1 == 1 {
          cells.flip(self.to_index(x, y))
        }
      }
    }

    self.cells = cells;
  }

  pub fn render(&self) {
    // clear terminal
    print!("\x1b[2J\x1b[1;1H");

    for y in (0..self.height).step_by(4) {
      for x in (0..self.width).step_by(2) {
        let byte = BRAILLE.iter()
          .filter(|((u, v), _)| self.at(x + u, y + v).is_alive())
          .map(|(_, bit)| bit)
          .fold(0, |a, b| a | b);

        let c = 0x2800 | byte as u32;
        let c = unsafe { std::char::from_u32_unchecked(c) };
        print!("{}", c);
      }

      io::stdout().flush().unwrap();
    }
  }
}
