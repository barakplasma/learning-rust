fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut team_splitter = [true, false].iter().cycle();
    let teams: (Vec<u32>, Vec<u32>) = array.iter().partition(|_| {
        *team_splitter.next().unwrap()
    });
    (teams.0.iter().sum(), teams.1.iter().sum())
}

#[test]
fn basic_tests() {
  assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
  assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
  assert_eq!(row_weights(vec![80]), (80,0));
}