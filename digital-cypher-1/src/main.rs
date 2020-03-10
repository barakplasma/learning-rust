fn main() {
    println!("Hello, world!");
    encode("scout".to_string(), 1939);
}
fn encode(msg: String, n: i32) -> Vec<i32> {
    // Digital Cypher assigns to each letter of the alphabet unique number.
    let alphabet: Vec<(usize, char)> = "abcdefghijklmnopqrstuvwxyz".chars().enumerate().collect();
    // Instead of letters in encrypted word we write the corresponding number
    let msg_indexes: Vec<i32> = msg.chars().fold(Vec::new(), |mut acc, c| {
        if let Some(i) = alphabet.iter().find(|l| l.1 == c) {
            acc.push((i.0+1) as i32);
        }
        acc
    });
    //Then we add to each obtained digit consecutive digits from the key.
    let key: Vec<char> = n.to_string().chars().collect();
    let mut cycle_key = key.iter().cycle();
    msg_indexes.iter().map(|i| {
        i + cycle_key.next().unwrap().to_digit(10).unwrap() as i32
    }).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }
}