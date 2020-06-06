pub fn xorshift(seed: u64) -> impl FnMut() -> u64 {
  let mut n = seed;
  move || {
    n ^= n << 13;
    n ^= n >> 7;
    n ^= n << 17;
    n
  }
}
