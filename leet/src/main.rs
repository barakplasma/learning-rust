fn main() {
    to_leet_speak("LEET");
}

fn to_leet_speak(s: &str) -> String {
    let alphabet = [
        ['A', '@'],
        ['B', '8'],
        ['C', '('],
        ['E', '3'],
        ['G', '6'],
        ['H', '#'],
        ['I', '!'],
        ['L', '1'],
        ['O', '0'],
        ['S', '$'],
        ['T', '7'],
        ['Z', '2'],
    ];
    s.chars()
        .map(|l| {
            alphabet
            .iter()
            .find(|&&x| x[0] == l)
            .or(std::option::Option::Some(&[l,l]))
            .unwrap()[1]
        })
        .collect()
}

#[test]
fn leet() {
    assert_eq!(to_leet_speak("LEET"), "1337");
}

#[test]
fn codewars() {
    assert_eq!(to_leet_speak("CODEWARS"), "(0D3W@R$");
}

#[test]
fn hello_world() {
    assert_eq!(to_leet_speak("HELLO WORLD"), "#3110 W0R1D");
}

#[test]
fn lorem_ipsum() {
    assert_eq!(
        to_leet_speak("LOREM IPSUM DOLOR SIT AMET"),
        "10R3M !P$UM D010R $!7 @M37"
    );
}

#[test]
fn quick_brown_fox() {
    assert_eq!(
        to_leet_speak("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"),
        "7#3 QU!(K 8R0WN F0X JUMP$ 0V3R 7#3 1@2Y D06"
    );
}
