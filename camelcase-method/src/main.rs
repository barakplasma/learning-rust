fn capitalize(x: &str) -> String {
  let (first, last) = x.split_at(1);
  let mut upper_case_first_character = first.to_uppercase();
  upper_case_first_character.push_str(last);
  return upper_case_first_character;
}

fn camel_case(str: &str) -> String {
    str.to_string().split(' ').filter(|s| s.len() > 0).map(capitalize).collect()
}

// Rust tests
#[test]
fn simple_test() {
  assert_eq!(camel_case("test case"), "TestCase");
  assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
}

#[test]
fn complex_test() {
  assert_eq!(camel_case("say hello "), "SayHello");
  assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
  assert_eq!(camel_case(""), "");
}

fn main() {
  camel_case("test case");
}