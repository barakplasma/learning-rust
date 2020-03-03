#[no_mangle]
pub fn answer() -> i32 {
  42
}

#[no_mangle]
pub fn sum_rs(n: i32) -> i32 {
  (0..n+1).sum()
}