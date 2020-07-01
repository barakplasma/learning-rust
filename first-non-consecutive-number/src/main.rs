fn main() {
    println!("Hello, world!");
    first_non_consecutive(&vec![1,2,3,4,6,7,8]);
}

// https://www.codewars.com/kata/58f8a3a27a5c28d92e000144/solutions/rust
fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    match arr.windows(2).find(|w| {
            w[0].ne(&(w[1] - 1))
    }) {
        Some(x) => Some(x[1]),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,6,7,8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,5,6,7,8]), None);
        assert_eq!(first_non_consecutive(&vec![4,6,7,8,9,11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4,5,6,7,8,9,11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31,32]), None);
        assert_eq!(first_non_consecutive(&vec![-3,-2,0,1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5,-4,-3,-1]), Some(-1));
    }
}
