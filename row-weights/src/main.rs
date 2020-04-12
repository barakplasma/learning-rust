fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut teams = (0u32, 0u32);
    for i in 0..array.len() {
        match i % 2 {
            0 => {teams.0+=array[i]},
            1 => {teams.1+=array[i]},
            _ => panic!("unexpected result")
        }
    }
    teams
}

#[test]
fn basic_tests() {
  assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
  assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
  assert_eq!(row_weights(vec![80]), (80,0));
}