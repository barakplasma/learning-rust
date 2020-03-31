fn main() {
    // https://www.codewars.com/kata/5a023c426975981341000014/train/rust
    println!("third internal angle of a triangle");
    other_angle(30, 60);
}
// clever enough?
struct Polygon {
    total_of_internal_angles: u32,
    side_lengths: Vec<u32>
}

impl Polygon {
    fn sum_internal_angles(self)->u32 {
        self.side_lengths.into_iter().sum()
    }
    fn missing_angle(self)->u32 {
        self.total_of_internal_angles - self.sum_internal_angles()
    }
}

fn other_angle(a: u32, b: u32) -> u32 {
    let triangle = Polygon { 
        total_of_internal_angles: 180,
        side_lengths: vec![a, b]
    };
    triangle.missing_angle()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(other_angle(30, 60), 90);
        assert_eq!(other_angle(60, 60), 60);
        assert_eq!(other_angle(43, 78), 59);
        assert_eq!(other_angle(10, 20), 150);
    }
}
