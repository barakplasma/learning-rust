fn main() {
    println!("rectangle into squares: ");
    sq_in_rect(5, 3);
}

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng.eq(&wdth) {
        None
    } else {
        let min = lng.min(wdth);
        let max = lng.max(wdth);
        let mut output = vec![min];
        let mut next = sq_in_rect(max - min, min).unwrap_or(vec![min]);
        output.append(&mut next);
        Some(output)
    }
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
  
}