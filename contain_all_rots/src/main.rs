fn main() {
    println!("Hello, world!");
    contain_all_rots("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"]);
}

// https://www.codewars.com/kata/5700c9acc1555755be00027e/train/rust
fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    let omni = strng.repeat(2);
    let len = strng.len();
    (0..len).all(|offset| arr.contains(&&omni[offset..offset+len]))
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(strng: &str, arr: Vec<&str>, exp: bool) -> () {
        println!("strng: {}", strng);
        println!("arr: {:?}", arr);
        let ans = contain_all_rots(strng, arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest("", vec![], true);
    }
    #[test]
    fn basis_tests2() {
        dotest("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"], true);
    }
    #[test]
    fn basis_tests3() {
        dotest("XjYABhR", vec!["TzYxlgfnhf", "yqVAuoLjMLy", "BhRXjYA", "YABhRXj", "hRXjYAB", "jYABhRX", "XjYABhR", "ABhRXjY"], false);
    }
}