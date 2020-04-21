fn main() {
    even_numbers(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
}

fn is_even<'r>(n: &'r &i32) -> bool {
    n.checked_rem(2) == Some(0)
}
// https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c/train/rust
fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let mut last_number_even_numbers_from_array: Vec<i32> = vec![];
    let mut even_number_iter = array.iter().filter(is_even);
    for _ in 0..number {
        last_number_even_numbers_from_array.push(*even_number_iter
            .next_back().unwrap())
    }
    last_number_even_numbers_from_array.reverse();
    last_number_even_numbers_from_array
}

#[test]
fn sample_tests() {
    assert_eq!(
        even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3),
        vec!(4, 6, 8)
    );
    assert_eq!(
        even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2),
        vec!(-8, 26)
    );
    assert_eq!(
        even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1),
        vec!(6)
    );
}
