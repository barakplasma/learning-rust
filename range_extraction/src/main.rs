fn main() {
    println!("Hello, world!");
    solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]);
}

mod solution {
    
    pub fn range_extraction(a: &[i32]) -> String {
        a.iter().enumerate().fold(String::new(), |acc,(i,curr)| {
            if i==0 {return curr.to_string();}
            if *a.get(i.checked_sub(1_usize).unwrap_or_default()).or(Some(&std::i32::MAX)).unwrap() == curr-1 && *a.get(i+1_usize).or(Some(&std::i32::MAX)).unwrap() == curr+1 {return acc.to_string();}
            if *a.get(i.checked_sub(2_usize).unwrap_or_default()).or(Some(&std::i32::MAX)).unwrap() == curr-2 && *a.get(i-1_usize).or(Some(&std::i32::MAX)).unwrap() == curr-1 {return acc.to_string()+"-"+&curr.to_string();}
            return acc.to_string()+","+&curr.to_string();
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
        assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
    }
}
