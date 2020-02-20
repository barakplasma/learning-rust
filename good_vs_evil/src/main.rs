fn main() {
    println!("Hello, world!");
    good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0");
}

fn get_worth(side: &str, worth: [i32; 7]) -> i32 {
    let mut sum = 0;
    let mut index = 0;
    let counts = side.split(" ").map(|c| c.parse::<i32>().unwrap());
    for count in counts {
        sum += worth[index] * count;
        index += 1;
    }
    sum
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let battle_result = get_worth(good, [1,2,3,3,4,10,0]) - get_worth(evil, [1,2,2,2,3,5,10]);
    
    match battle_result {
        std::i32::MIN..=-1 => String::from("Battle Result: Evil eradicates all trace of Good"),
        0 => String::from("Battle Result: No victor on this battle field"),
        1..=std::i32::MAX => String::from("Battle Result: Good triumphs over Evil"),
    }
}

#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

// function goodVsEvil(good, evil) {  
//     var getWorth = function( side, worth ) {
//       return side.split(' ').reduce( function(result, value, index) { 
//         return result + (worth[index] * value);
//       }, 0);
//     }
  
//     var result = getWorth( good, [1,2,3,3,4,10] ) - getWorth( evil, [1,2,2,2,3,5,10] );
  
//     return result > 0 ? "Battle Result: Good triumphs over Evil" :
//            result < 0 ? "Battle Result: Evil eradicates all trace of Good" :
//                         "Battle Result: No victor on this battle field";
//   }