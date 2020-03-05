fn main() {
    println!("Hello, world!");
    let res = find_sum_of_multiples_of_three_and_five(std::i32::MAX);
    println!("{:?}", res);
}
// 1229782937674641134
fn find_sum_of_multiples_of_three_and_five(n: i32) -> i64 {
    let upto: i64 = n as i64;
    let threes: i64 = (0..=upto).step_by(3).sum();
    let fives: i64 = (0..=upto).step_by(5).sum();
    threes + fives
}