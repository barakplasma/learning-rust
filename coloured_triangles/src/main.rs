fn main() {
    println!("Hello, world!");
    triangle("GB");
}

fn child_color(chunk: &[char]) -> char {
    match chunk {
        ['R', 'R'] => 'R',
        ['G', 'G'] => 'G',
        ['B', 'B'] => 'B',
        ['R', 'G'] => 'B',
        ['G', 'R'] => 'B',
        ['B', 'G'] => 'R',
        ['G', 'B'] => 'R',
        ['R', 'B'] => 'G',
        ['B', 'R'] => 'G',
        _ => 'E',
    }
}

fn triangle(row_str: &str) -> String {
    let colors = row_str.chars();
    if row_str.chars().count() == 1 {return String::from(row_str)}
    let colors_vec: Vec<char> = colors.collect();

    let color_chunks: std::slice::Windows<char> = colors_vec.windows(2);

    let mut output = String::from("");

    for chunk in color_chunks {
        output.push(child_color(chunk));
    }
    if output.chars().count() > 1 {
        output = triangle(&output);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(triangle("GB"), "R");
    }
    #[test]
    fn test_triangle1() {
        assert_eq!(triangle("RGBG"), "B");
    }
    #[test]
    fn test_triangle2() {
        assert_eq!(triangle("RRR"), "R");
    }
    #[test]
    fn test_triangle3() {
        assert_eq!(triangle("RBRGBRB"), "G");
    }
    #[test]
    fn test_triangle4() {
        assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
    }
    #[test]
    fn test_triangle5() {
        assert_eq!(triangle("GB"), "R");
    }
    #[test]
    fn test_triangle6() {
        assert_eq!(triangle("B"), "B");
    }
}
