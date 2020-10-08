use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let decoder = MorseDecoder::new();
    decoder.decode_morse(".... . -.--   .--- ..- -.. .");
}

// Preloaded:
//
struct MorseDecoder {
    morse_code: HashMap<String, String>,
}
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                (".-", "A"),
                ("-...", "B"),
                ("-.-.", "C"),
                ("-..", "D"),
                (".", "E"),
                ("..-.", "F"),
                ("--.", "G"),
                ("....", "H"),
                ("..", "I"),
                (".---", "J"),
                ("-.-", "K"),
                (".-..", "L"),
                ("--", "M"),
                ("-.", "N"),
                ("---", "O"),
                (".--.", "P"),
                ("--.-", "Q"),
                (".-.", "R"),
                ("...", "S"),
                ("-", "T"),
                ("..-", "U"),
                ("...-", "V"),
                (".--", "W"),
                ("-..-", "X"),
                ("-.--", "Y"),
                ("--..", "Z"),
                ("-----", "0"),
                (".----", "1"),
                ("..---", "2"),
                ("...--", "3"),
                ("....-", "4"),
                (".....", "5"),
                ("-....", "6"),
                ("--...", "7"),
                ("---..", "8"),
                ("----.", "9"),
                (".-.-.-", "."),
                ("--..--", ","),
                ("..--..", "?"),
                (".----.", "\'"),
                ("-.-.--", "!"),
                ("-..-.", "/"),
                ("-.--.", "("),
                ("-.--.-", ")"),
                (".-...", "&"),
                ("---...", ","),
                ("-.-.-.", ";"),
                ("-...-", "="),
                (".-.-.", "+"),
                ("-....-", "-"),
                ("..--.-", "_"),
                (".-..-.", "\""),
                ("...-..-", "$"),
                (".--.-.", "@"),
                ("...---...", "SOS"),
            ]
            .iter()
            .map(|&(from, to)| (from.to_string(), to.to_string()))
            .collect(),
        }
    }

    fn decode_morse(&self, encoded: &str) -> String {
        let space = " ".to_string();
        let nospace = "".to_string();
        return encoded
            .split(" ")
            .scan(false, |previous, l| match previous {
                true => {
                    *previous = false;
                    Some(self.morse_code.get(l).unwrap_or(&nospace).as_str())
                }
                false => {
                    let letter = self.morse_code.get(l);
                    if letter == None {
                        *previous = true;
                    }
                    Some(letter.unwrap_or(&space).as_str())
                }
            })
            .collect::<Vec<&str>>()
            .join("")
            .trim()
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }
    #[test]
    fn test_spaces() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(""), "");
        assert_eq!(decoder.decode_morse("   "), "");
    }

    #[test]
    fn test01_basic() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(""), "");
        assert_eq!(decoder.decode_morse(".-"), "A");
        assert_eq!(decoder.decode_morse("."), "E");
        assert_eq!(decoder.decode_morse(".."), "I");
        assert_eq!(decoder.decode_morse(". ."), "EE");
        assert_eq!(decoder.decode_morse(".   ."), "E E");
        assert_eq!(decoder.decode_morse("...---..."), "SOS");
        assert_eq!(decoder.decode_morse("... --- ..."), "SOS");
        assert_eq!(decoder.decode_morse("...   ---   ..."), "S O S");
    }

    #[test]
    fn test02_extra_space() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("   "), "");
        assert_eq!(decoder.decode_morse(" . "), "E");
        assert_eq!(decoder.decode_morse("   .   . "), "E E");
        assert_eq!(decoder.decode_morse(".   .   "), "E E");
    }

    #[test]
    fn test03_complex() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("      ...---... -.-.--   - .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. ...   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. .-.-.-     "), "SOS! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.");
    }
}
