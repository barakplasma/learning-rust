fn main() {
    println!("Hello, world!");
    part_list(vec!["cdIw", "tzIy", "xDu", "rThG"]);
}
// https://www.codewars.com/kata/56f3a1e899b386da78000732/solutions/rust
fn part_list(arr: Vec<&str>) -> String {
    let mut out = String::new();

    for i in 1..arr.len() {
        let (head, tail) = arr.split_at(i);
        out += &format!("({}, {})", head.join(" "), tail.join(" "));
    }

    return out;
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn complex_tests() {
        dotest(vec!["cdIw", "tzIy", "xDu", "rThG"], 
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
        
    }

    #[test]
    fn basic_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
    }
}
