// https://www.codewars.com/kata/reviews/58cacb357584f5e6e5000121/groups/5e579ea59ae1350001f7d070
// https://www.codewars.com/kata/540afbe2dc9f615d5e000425/solutions/rust 

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let sudoku_example = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };

    sudoku_example.is_valid();
}

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        if !self.has_valid_rows() {
            return false;
        }
        if !self.has_valid_cols() {
            return false;
        }
        if !self.has_valid_boxes() {
            return false;
        }
        return true;
    }

    fn check_all_rows(&self, data: &Vec<Vec<u32>>) -> bool {
        data.iter().all(|row| self.is_valid_row(row))
    }
    fn has_valid_rows(&self) -> bool {
        self.check_all_rows(&self.data)
    }

    fn transpose(&self) -> Vec<Vec<u32>> {
        let mut columns: Vec<Vec<u32>> = vec![];
        for col_index in 0..self.max_row_length() {
            let mut new_col: Vec<u32> = vec![];
            for row in &self.data {
                new_col.push(row[col_index]);
            }
            columns.push(new_col);
        }
        columns
    }

    fn has_valid_cols(&self) -> bool {
        let transposed_board = self.transpose();
        self.check_all_rows(&transposed_board)
    }

    fn has_valid_boxes(&self) -> bool {
        let box_size = f64::from(self.max_row_length() as u32).sqrt() as usize;
        let max_length = self.max_row_length();
        let mut boxes: Vec<Vec<u32>> = vec![];
        let mut row_i: usize = 0;
        let mut col_i: usize = 0;
        let mut box_ri: usize;
        let mut box_ci: usize;
        while row_i < max_length {
            while col_i < max_length {
                let mut new_box: Vec<u32> = vec![];
                box_ri = row_i;
                while box_ri < (row_i + box_size) {
                    box_ci = col_i;
                    while box_ci < (col_i + box_size) {
                        new_box.push(self.data[box_ri][box_ci]);
                        box_ci += 1;
                    }
                    box_ri += 1;
                }
                col_i += box_size;
                boxes.push(new_box);
            }
            row_i += box_size;
        }
        boxes.iter().all(|b| self.is_valid_row(b))
    }

    fn max_row_length(&self) -> usize {
        let mut max = self.data.len();
        for row in &self.data {
            let cur_len = row.len();
            if cur_len > max {
                max = cur_len;
            }
        }
        return max;
    }

    fn is_valid_row(&self, row: &Vec<u32>) -> bool {
        let valid_row = 1..(self.max_row_length() + 1) as u32;

        fn add_to_hash_set(mut hs: HashSet<u32>, n: u32) -> HashSet<u32> {
            hs.insert(n);
            hs
        }

        let valid_row_set: HashSet<u32> = valid_row.fold(HashSet::new(), add_to_hash_set);

        let row_hash_set: HashSet<u32> = row.iter().fold(HashSet::new(), |mut hs, &n| {
            hs.insert(n);
            hs
        });

        let difference = valid_row_set.difference(&row_hash_set);

        if difference.count() == 0 {
            return true;
        } else {
            false
        }
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };
    assert!(!bad_sudoku_1.is_valid());
}
#[test]
fn bad_sudoku2() {
    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_2.is_valid());
}
#[test]
fn bad_sudoku3() {
    let bad_sudoku_3 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![4, 1, 3, 2],
            vec![3, 2, 4, 1],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(!bad_sudoku_3.is_valid());
}
