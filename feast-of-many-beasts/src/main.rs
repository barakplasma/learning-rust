fn main() {
    println!("Hello, world!");
    feast("great blue heron", "garlic naan");
}

// https://www.codewars.com/kata/reviews/5bead1ac56b1460356001681/groups/5e843a4f077c3500016ce0e4
fn feast(beast: &str, dish: &str) -> bool {
    let mut dish_iter = dish.chars();
    let dish_starts_with = dish_iter.next().unwrap();
    let dish_ends_with = dish_iter.next_back().unwrap();
    beast.starts_with(dish_starts_with) && beast.ends_with(dish_ends_with)
}

// Rust test example:
#[test]
fn sample_test_cases1() {
  assert_eq!(feast("great blue heron", "garlic naan"), true);
}
#[test]
fn sample_test_cases2() {
  assert_eq!(feast("chickadee", "chocolate cake"), true);
}
#[test]
fn sample_test_cases3() {
  assert_eq!(feast("chickadef", "chocolate cake"), false);
}
#[test]
fn sample_test_cases4() {
  assert_eq!(feast("brown bear", "bear claw"), false);
}