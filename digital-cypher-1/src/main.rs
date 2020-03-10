fn main() {
    println!("Hello, world!");
}
use std::collections::HashMap;

fn encode(msg: String, n: i32) -> Vec<i32> {
    // Digital Cypher assigns to each letter of the alphabet unique number.
    let alphabet = "abcdefghijklmnopqrstuvwxyz".char_indices();
    let char_to_index : HashMap<char, i32> = alphabet
        .into_iter()
        .fold(
            HashMap::new(), 
        |mut acc, p| {
            acc.insert(p.1, (p.0 + 1) as i32 ); 
            acc
        });
    // Instead of letters in encrypted word we write the corresponding number
    let msg_indexes: Vec<i32> = msg.chars().fold(Vec::new(), |mut acc, c| {
        if let Some(i) = char_to_index.get(&c) {
            acc.push(*i);
        }
        acc
    });
    //Then we add to each obtained digit consecutive digits from the key.
    let key = format!("{}", n).chars().fold(Vec::new(), |mut acc, c: char| {
        acc.push(c.to_digit(10).unwrap() as i32);
        acc
    });
    let mut cycle_key = key.iter().cycle();
    msg_indexes.into_iter().map(|i| {
        i+cycle_key.next().unwrap()
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