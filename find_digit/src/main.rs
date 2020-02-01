// Complete the function that takes two numbers as input, num and nth and return the nth digit of num (counting from right to left).

// Note
// If num is negative, ignore its sign and treat it as a positive value
// If nth is not positive, return -1
// Keep in mind that 42 = 00042. This means that findDigit(42, 5) would return 0

fn main() {
    println!("Hello, world!");
    find_digit(5673, 4);
}

fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    };
    let mut digits = num.abs().to_string();
    if nth as usize > digits.len() {
        return 0;
    };
    let d = digits.remove(digits.len() - nth as usize);

    d.to_digit(10).unwrap() as i32
}

#[test]
fn example_test() {
    assert_eq!(find_digit(5673, 4), 5);
    assert_eq!(find_digit(129, 2), 2);
    assert_eq!(find_digit(-2825, 3), 8);
    assert_eq!(find_digit(-456, 4), 0);
    assert_eq!(find_digit(0, 20), 0);
    assert_eq!(find_digit(65, 0), -1);
    assert_eq!(find_digit(24, -8), -1);
}
