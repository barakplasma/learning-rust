/**

"You are provided two arrays that contain strings. Find the
greatest difference between the longest string in one array
and the shortest string of the other array."


function mxdiflg(a1, a2) {
    const length = x => x.length;
    const lengths = x => [...x].map(length);

    if (a1.length <= 0 || a2.length <= 0) return -1;

    const a1s = lengths(a1);
    const a2s = lengths(a2);
    
    const checkPossibility = (first, second) => Math.abs(Math.min.apply(null, first) - Math.max.apply(null, second))

    const possibilityA = checkPossibility(a1s, a2s)
    const possibilityB = checkPossibility(a2s, a1s)

    return Math.max(possibilityA, possibilityB)
}
 */

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() <= 0 || a2.len() <= 0 {
        return -1;
    }
    fn lengths(a: Vec<&str>) -> Vec<usize> {a.into_iter().map(|word| word.chars().count() ).collect()}

    let a1l = lengths(a1);
    let a2l = lengths(a2);
    let a3l = a1l.clone();
    let a4l = a2l.clone();

    fn check_possibility(first: Vec<usize>, second: Vec<usize>) -> usize {
        let max = match first.into_iter().max() {
            Some(x) => x,
            None => usize::max_value()
        };
        let min = match second.into_iter().min() {
            Some(x) => x,
            None => usize::min_value()
        };
        max - min
    }

    let possibility_a = check_possibility(a1l, a2l);
    let possibility_b = check_possibility(a4l, a3l);

    let max = possibility_a.max(possibility_b) as i32;
    
    return max.abs()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn test_1() {
        let s1 = vec!["hoqq", "bbllkw", "oox", "ejjuyyy", "plmiis", "xxxzgpsssa", "xxwwkktt", "znnnnfqknaz", "qqquuhii", "dvvvwz"];
        let s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
    }
    
    #[test]
    fn test_2() {
        let s1 = vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"];
        let s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }

    #[test]
    fn test_should_be_negative() {
        let s1 = vec![];
        let s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, -1);
    }
}
