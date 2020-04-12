fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    array.iter().enumerate().fold((0u32, 0u32), |mut acc, item| {
        let index = item.0;
        match index % 2 {
            0 => acc.0 += item.1,
            1 => acc.1 += item.1,
            _ => panic!("unexpected modulus result")
        }
        acc
    })
}

#[test]
fn basic_tests() {
  assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
  assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
  assert_eq!(row_weights(vec![80]), (80,0));
}