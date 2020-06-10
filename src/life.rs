use super::bitfield::Bitfield;
use super::xorshift::Xorshift;
use std::array::IntoIter;
use std::iter;

const RULES: [u16; 2] = [
  // 876543210 neighbors alive
  0b_000001000, // 1 - dead cell becomes alive
  0b_000001100  // 1 - alive cell stays alive
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
  pub fn randomized(width: usize, height: usize, seed: u64) -> Self {
    let mut xs = Xorshift::new(seed);

    let bytes = iter::from_fn(|| Some(xs.next_u64()))
      .flat_map(|n| IntoIter::new(n.to_ne_bytes()))
      .take(width * height >> 3)
      .collect();

    let cells = Bitfield::from_bytes(bytes);
    Self { width, height, cells }
  }

  pub fn to_index(&self, x: usize, y: usize) -> usize {
    debug_assert!(x < self.width && y < self.height);
    self.width * y + x
  }

  pub fn is_alive(&self, x: usize, y: usize) -> bool {
    self.cells.get(self.to_index(x, y))
  }

  pub fn neighbors(&self, x: usize, y: usize) -> usize {
    debug_assert!(x < self.width && y < self.height);

    NEIGHBORS.iter()
      .filter(move |(u, v)| {
        let x = (x + u + self.width  - 1) % self.width;
        let y = (y + v + self.height - 1) % self.height;
        self.is_alive(x, y)
      })
      .map(|_| 1)
      .sum()
  }

  pub fn step(&mut self) {
    let mut cells = Bitfield::new(self.cells.size());

    for y in 0..self.height {
      for x in 0..self.width {
        let c = self.is_alive(x, y);
        let n = self.neighbors(x, y);
        if RULES[c as usize] >> n & 1 == 1 {
          cells.flip(self.to_index(x, y))
        }
      }
    }

    self.cells = cells;
  }

  pub fn render(&self) -> String {
    (0..self.height).step_by(4).flat_map(|y| {
      (0..self.width).step_by(2).map(move |x| {
        let byte = BRAILLE.iter()
          .filter(move |((u, v), _)| self.is_alive(x + u, y + v))
          .map(|(_, bit)| bit)
          .fold(0, |a, b| a | b);

        let code = 0x2800 | byte as u32;
        unsafe { std::char::from_u32_unchecked(code) }
      })
    }).collect()
  }
}
