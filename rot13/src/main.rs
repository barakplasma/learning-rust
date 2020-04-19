use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn rot13(message: &str) -> String {
    let mut cipher: HashMap<char, char> = [('a', 'n'), ('b', 'o'), ('c', 'p'), ('d', 'q'), ('e', 'r'), ('f', 's'), ('g', 't'), ('h', 'u'), ('i', 'v'), ('j', 'w'), ('k', 'x'), ('l', 'y'), ('m', 'z'), ('n', 'a'), ('o', 'b'), ('p', 'c'), ('q', 'd'), ('r', 'e'), ('s', 'f'), ('t', 'g'), ('u', 'h'), ('v', 'i'), ('w', 'j'), ('x', 'k'), ('y', 'l'), ('z', 'm'), ('A', 'N'), ('B', 'O'), ('C', 'P'), ('D', 'Q'), ('E', 'R'), ('F', 'S'), ('G', 'T'), ('H', 'U'), ('I', 'V'), ('J', 'W'), ('K', 'X'), ('L', 'Y'), ('M', 'Z'), ('N', 'A'), ('O', 'B'), ('P', 'C'), ('Q', 'D'), ('R', 'E'), ('S', 'F'), ('T', 'G'), ('U', 'H'), ('V', 'I'), ('W', 'J'), ('X', 'K'), ('Y', 'L'), ('Z', 'M')].iter().cloned().collect();
    for c in message.chars() {
        cipher.entry(c).or_insert(c);
    }
    let encoded: String = message.chars().map(|message_char| {
        match cipher.get(&message_char) {
        Some(c) => c,
        None => &'a'
    }}).collect();
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
    #[test]
    fn test_fixed2() {
        assert_eq!(rot13("test "), "grfg ");
        assert_eq!(rot13("Test!"), "Grfg!");
        assert_eq!(rot13("Test1"), "Grfg1");
    }
}