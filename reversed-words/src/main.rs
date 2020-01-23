use std::thread;
use std::sync::mpsc::channel;

fn main() {
    println!("{}", reverse_words(&"hello world"));
}

fn reverse_words(str: &'static str) -> String {
    let words = str.split(" ");
    let mut output: Vec<String> = vec![];
    
    for word in words {
        // Create a shared channel that can be sent along from many threads
        // where tx is the sending half (tx for transmission), and rx is the receiving
        // half (rx for receiving).
        let (tx, rx) = channel();
        thread::spawn(move|| {
            let wr = reverse_one_word(word);
            tx.send(wr).unwrap();
        });
        for received in rx {
            output.push(received);
        }
    }
    
    output.join(" ")
}

fn reverse_one_word(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

// Rust tests
#[test]
fn sample_test() {
  assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
  assert_eq!(reverse_words("apple"), "elppa");
  assert_eq!(reverse_words("a b c d"),"a b c d");
  assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}