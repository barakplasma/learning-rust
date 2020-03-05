#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn main() {
    println!("Hello, world!");
    let res = find_sum(std::i16::MAX);
    println!("{:?}", res);
}
// 1229782937674641134

fn sum_of_arithmetic_sequence(n: i64, d: i64) -> i64 {
    let x = n / d; 
    return d * x * (x + 1) / 2; 
}
fn find_sum(n: i16) -> i64 {
    sum_of_arithmetic_sequence(n as i64, 3) + sum_of_arithmetic_sequence(n as i64, 5) - sum_of_arithmetic_sequence(n as i64, 15)
}

#[allow(dead_code)]
fn sum_of_arithmetic_sequence_ajfaldjfalf(n: i64, d: i64) -> i64 {
    let x = n / d; 
    return d * x * (x + 1) / 2; 
}

#[allow(dead_code)]
fn find_sum_solution_11413413(n: i16) -> i64 {
    sum_of_arithmetic_sequence_ajfaldjfalf(n as i64, 3) + sum_of_arithmetic_sequence_ajfaldjfalf(n as i64, 5) - sum_of_arithmetic_sequence_ajfaldjfalf(n as i64, 15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum_5() {
        assert_eq!(find_sum(5), 8);
    }
    #[test]
    fn test_find_sum_10() {
        assert_eq!(find_sum(10), 33);
    }
    #[test]
    fn test_find_sum_100() {
        assert_eq!(find_sum(100), 2418);
    }
    #[test]
    fn test_find_sum_1000() {
        assert_eq!(find_sum(1000), 234168);
    }
    #[test]
    fn test_find_sum_largest_valid_input() {
        assert_eq!(find_sum(std::i16::MAX), 250532114);
    }

    quickcheck! {
        fn test_find_sum_quickcheck(n: i16) -> bool {
            let n2 = n.clone();
            find_sum(n) == find_sum_solution_11413413(n2)
        }
    }
}