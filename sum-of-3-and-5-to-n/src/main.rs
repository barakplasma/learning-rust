#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn main() {
    println!("Hello, world!");
    let res = find_sum(std::u32::MAX);
    println!("{:?}", res);
}

fn sum_of_arithmetic_sequence(n: u32, d: u32) -> u64 {
    let x: u64 = (n / d) as u64; 
    return d as u64 * x * (x + 1) / 2; 
}

fn find_sum(n: u32) -> u64 {
    sum_of_arithmetic_sequence(n, 3) + sum_of_arithmetic_sequence(n, 5) - sum_of_arithmetic_sequence(n, 15)
}

#[allow(dead_code)]
fn sum_of_arithmetic_sequence_ajfaldjfalf(n: u32, d: u32) -> u64 {
    let x: u64 = (n / d) as u64; 
    return d as u64 * x * (x + 1) / 2; 
}

#[allow(dead_code)]
fn find_sum_solution_11413413(n: u32) -> u64 {
    sum_of_arithmetic_sequence(n, 3) + sum_of_arithmetic_sequence(n, 5) - sum_of_arithmetic_sequence(n, 15)
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
        assert_eq!(find_sum(std::u32::MAX), 4304240284008727620);
    }

    quickcheck! {
        fn test_find_sum_quickcheck(n: u32) -> bool {
            let n2 = n.clone();
            find_sum(n) == find_sum_solution_11413413(n2)
        }
    }
}