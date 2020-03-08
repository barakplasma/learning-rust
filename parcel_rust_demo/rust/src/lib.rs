#[no_mangle]
pub fn answer() -> u32 {
  42
}

#[no_mangle]
pub fn sum_rs(n: u32) -> u32 {
  (0..n+1).sum()
}
