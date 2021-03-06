use std::fmt;

fn main() {
    revrot("1234", 0);
}

/// reverses or rotates chunks of a string
/// https://www.codewars.com/kata/56b5afb4ed1f6d5fb0000991/train/rust
fn revrot(s: &str, sz: usize) -> String {
    struct DigitChunk {
        digits: Vec<u32>,
    }
    impl DigitChunk {
        fn join_digits(&self, with: &str) -> String {
            self.digits.iter().fold(String::new(), |acc, cur| acc + with + &cur.to_string())
        }

        pub fn new(chars: Vec<char>) -> Self {
            let mut digits: Vec<u32> = chars.iter().map(|y| {
                y.to_digit(10).unwrap()
            }).collect();
            let sum_of_cubes: u32 = digits.iter().map(|x| x.pow(3)).sum();
            let divides_by_two: bool = sum_of_cubes % 2 == 0;
            if divides_by_two {
                digits.reverse();
            } else {
                digits.rotate_left(1);
            }
            Self {
                digits
            }
        }
    }
    impl fmt::Display for DigitChunk {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "digits: {}\n", self.join_digits(" "))
        }
    }
    let mut output = String::new();
    if usize::gt(&sz, &usize::min_value()) && s.chars().count() > 0  {
        let chunk_vector: Vec<DigitChunk> = s.chars()
        .collect::<Vec<char>>()
        .chunks_exact_mut(sz)
        .map(|w| DigitChunk::new(w.to_vec()))
        .collect();
        // for chunk in &chunk_vector {
        //     println!("{}", chunk);
        // }
        output = chunk_vector.iter().fold(String::new(), |acc, chunk| acc + chunk.join_digits("").as_str());
    }


    return output;
}

fn testing(s: &str, c: usize, exp: &str) -> () {
    assert_eq!(&revrot(s, c), exp)
}

#[test]
fn empty_revrot() {
    testing("1234", 0, "");
    testing("", 0, "");
    testing("", 1, "");
}
#[test]
fn window_too_small() {
    testing("1234", 5, "");
}

#[test]
fn basic_revrot() {
    let s = "733049910872815764";
    testing(s, 5, "330479108928157");
}

#[test]
fn advanced_revrot() {
    let s = "73304991087281576455176044327690580265896";
    testing(s, 8, "1994033775182780067155464327690480265895");
}