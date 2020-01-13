use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
    let mut word_set = HashSet::new();
    let mut new_str = String::from("");
    for word in s.split_whitespace() {
        if word_set.insert(word) {
            if new_str.len() > 0 {new_str.push_str(" ");}
            new_str.push_str(word);
        }
    }
    new_str
}

fn main() {
    println!("Hello, world!");
}
// Rust test example:
#[test]
fn sample_test_cases() {
  assert_eq!(remove_duplicate_words("alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"), "alpha beta gamma delta");
  assert_eq!(remove_duplicate_words("my cat is my cat fat"), "my cat is fat");
}