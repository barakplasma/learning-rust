fn main() {
    println!("Hello, world!");
}

fn solve(arr: &[&str]) -> String {
    if arr.len() < 2 {
        return String::from("23:59");
    }
    let mut minutes: Vec<u64> = arr.iter().map(time_to_minutes).collect();
    minutes.sort_unstable();
    let mut max_difference: u64 = minutes.windows(2).map(|w| w[1] - w[0]).max().unwrap();
    let wrap_around = 24*60 - minutes.last().unwrap() + minutes.first().unwrap();
    if max_difference < wrap_around {
        max_difference = wrap_around;
    }
    minutes_to_time(max_difference - 1)
}

fn time_to_minutes(time: &&str) -> u64 {
    let parts: Vec<u64> = time.split(":").map(|x| x.parse::<u64>().unwrap()).collect();
    return parts[0] * 60 + parts[1];
}

fn minutes_to_time(minutes: u64)->String{
    format!("{hr:0>0$}:{min:0>0$}", 2, hr=minutes / 60, min=minutes % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(solve(&["14:51"]), "23:59");
        assert_eq!(solve(&["23:00", "04:22", "18:05", "06:24"]), "11:40");
        assert_eq!(
            solve(&["21:14", "15:34", "14:51", "06:25", "15:30"]),
            "09:10"
        );
    }
}
