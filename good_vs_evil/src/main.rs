fn main() {
    println!("Hello, world!");
    good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0");
}

fn get_worth(side: &str, worth: [u8; 7]) -> u8 {
    let counts = side.split(" ").map(|c| c.parse::<u8>().unwrap());
    counts.zip(worth.into_iter()).map(|tup| tup.0 * tup.1).sum()
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_worth_sum = get_worth(good, [1,2,3,3,4,10,0]);
    let evil_worth_sum = get_worth(evil, [1,2,2,2,3,5,10]);
    
    use std::cmp::Ordering;
    String::from(match good_worth_sum.cmp(&evil_worth_sum) {
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good",
        Ordering::Equal => "Battle Result: No victor on this battle field",
        Ordering::Greater => "Battle Result: Good triumphs over Evil",
    })
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