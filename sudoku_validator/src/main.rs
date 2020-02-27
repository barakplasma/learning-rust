// https://www.codewars.com/kata/reviews/58cacb357584f5e6e5000121/groups/5e579ea59ae1350001f7d070
// https://www.codewars.com/kata/540afbe2dc9f615d5e000425/solutions/rust

fn main() {
    let sudoku_example = sudoku_validator::Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };

    sudoku_example.is_valid();
}
