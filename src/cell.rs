pub enum Cell { Dead, Alive }

impl Cell {
  pub fn from_bool(b: bool) -> Self {
    match b {
      true  => Cell::Alive,
      false => Cell::Dead
    }
  }

  pub fn is_alive(&self) -> bool {
    matches!(self, Cell::Alive)
  }
}
