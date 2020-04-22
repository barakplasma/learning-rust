fn main() {
    play_pass("I LOVE YOU!!!", 1);
}

fn play_pass(s: &str, n: u32) -> String {    
    let mut reversable = s.chars().collect::<Vec<char>>()
        .iter()
        .map(|c| {
            if c.is_alphabetic() {
                fn find_in_alphabet(alphabet: &str, c: &char, n: u32) -> char {
                    let pos = alphabet.char_indices().position(|x| x.1 == *c);
                    let shift_by = n as usize;
                    let x = ((pos.unwrap()) + shift_by).checked_rem(26);
    
                    if let Some(index) = x  {
                        let new_char = alphabet.char_indices().nth(index);
                        if let Some(x) = new_char {
                            return x.1;
                        }
                    }
                    return *c;
                }
                if c.is_lowercase() {
                    return find_in_alphabet("abcdefghijklmnopqrstuvwxyz", &c, n);
                } else {
                    return find_in_alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &c, n);
                }
            }
            return *c;
        })
        .enumerate()
        .map(|c| {
            if c.0.checked_rem(2).unwrap() == 0 {
                c.1.to_uppercase().next().unwrap()
            } else {
                c.1.to_lowercase().next().unwrap()
            }
        })
        .map(|c| {
            if c.is_numeric() {
                return (9 - c.to_digit(10).unwrap()).to_string().as_mut_str().chars().next().unwrap();
            }
            return c;
        }).collect::<Vec<char>>();
    
    reversable.reverse();
    
    let mut output = String::new();
    for c in reversable {
        output.push(c);
    }
    output
}
#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        // dotest("MY GRANMA CAME FROM NY ON THE 23RD OF APRIL 2015", 2, "4897 NkTrC Hq fT67 GjV Pq aP OqTh gOcE CoPcTi aO");
        dotest("AAABBCCY", 1, "zDdCcBbB");
    }
}
