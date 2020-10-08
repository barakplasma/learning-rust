fn main() {
    let decoder = MorseDecoder::new();
    println!("Hello, world!");
    decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011");
    decoder.decode_morse("····");
}

use regex::Regex;
use std::collections::HashMap;

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

    fn time_unit_length(&self, encoded: &str) -> u8 {
        let mut time_unit_length: u8 = std::u8::MAX;
        let mut segment_length: u8 = 1;
        let mut previous_char: char = 's';
        for current_char in encoded.chars() {
            if previous_char != 's' {
                if current_char == previous_char {
                    segment_length = segment_length + 1;
                } else {
                    if time_unit_length > segment_length {
                        time_unit_length = segment_length;
                    }
                    segment_length = 1;
                }
            }
            previous_char = current_char;
        }
        time_unit_length
    }

    fn to_dots_and_dashes(&self, encoded: &str) -> String {
        let mut dots_and_dashes = String::new();
        for w in encoded
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .step_by(self.time_unit_length(encoded) as usize)
        {
            dots_and_dashes.push_str(match w {
                '1' => "-",
                '0' => ".",
                _ => "",
            });
        }
        dots_and_dashes
    }

    fn to_spaces_dots_and_dashes(&self, encoded: &str) -> String {
        let new_letter = Regex::new(r"[\.]{3}").unwrap();
        let new_word = Regex::new(r"[\.]{7}").unwrap();

        new_letter
            .replace_all(
                &new_word.replace_all(&self.to_dots_and_dashes(encoded), "   "),
                " ",
            )
            .to_string()
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        // println!("{:?}", encoded);
        // match encoded {
        //     "1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011" => "···· · −·−−   ·−−− ··− −·· ·".to_string(),
        //     x => x.to_string()
        // }
        self.to_spaces_dots_and_dashes(encoded.trim())
    }

    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|x| {
                x.split(' ')
                    .filter_map(|y| self.morse_code.get(y))
                    .cloned()
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    // fn decode_morse(&self, encoded: &str) -> String {
    //     let space = " ".to_string();
    //     let nospace = "".to_string();
    //     return encoded
    //         .split(" ")
    //         .scan(false, |previous, l| match previous {
    //             true => {
    //                 *previous = false;
    //                 Some(self.morse_code.get(l).unwrap_or(&nospace).as_str())
    //             }
    //             false => {
    //                 let letter = self.morse_code.get(l);
    //                 if letter == None {
    //                     *previous = true;
    //                 }
    //                 Some(letter.unwrap_or(&space).as_str())
    //             }
    //         })
    //         .collect::<Vec<&str>>()
    //         .join("")
    //         .trim()
    //         .to_string();
    // }
}

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse("····"), "H");
    // assert_eq!(decoder.decode_morse("···· · −·−−   ·−−− ··− −·· ·"), "HEY JUDE");
    // assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}