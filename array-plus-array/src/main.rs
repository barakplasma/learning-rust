fn main() {
    array_plus_array(vec![1], vec![4]);
}

fn array_plus_array(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    arr1.iter().sum::<i32>() + arr2.iter().sum::<i32>()
}

extern crate rand;
use crate::rand::Rng;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_plus_array() {
        assert_eq!(array_plus_array(vec![1], vec![4]), 5);
        assert_eq!(array_plus_array(vec![1, 2, 3], vec![4, 5, 6]), 21);
        assert_eq!(array_plus_array(vec![-1, -2, -3], vec![-4, -5, -6]), -21);
        assert_eq!(array_plus_array(vec![0, 0, 0], vec![4, 5, 6]), 15);
        assert_eq!(
            array_plus_array(vec![100, 200, 300], vec![400, 500, 600]),
            2100
        );
    }
    #[test]
    fn random_test_array_plus_array() {
        for _ in 0..100 {
            fn random() -> Vec<i32> {
                let mut rng = rand::thread_rng();
                let mut n: Vec<i32> = vec![];
                for _ in 0..100 {
                    n.push(rng.gen_range(0, 10));
                }
                n
            }
            let arr1 = random();
            let arr2 = random();
            let sum = arr1.iter().sum::<i32>() + arr2.iter().sum::<i32>();
            assert_eq!(array_plus_array(arr1, arr2), sum);
        }
    }
}
