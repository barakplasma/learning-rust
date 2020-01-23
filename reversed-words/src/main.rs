fn main() {
    println!("{}", reverse_words("hello world"));
}

fn reverse_words(str:&str) -> String {
    let words = str.split_whitespace();
    let mut sentence = words.collect::<Vec<&str>>();
    sentence.reverse();
    sentence.join(" ").to_string()
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
      assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}