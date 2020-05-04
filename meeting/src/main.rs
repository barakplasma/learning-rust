fn main() {
    println!("Hello, world!");
}

use std::cmp::Ordering;

struct Person {
    first_name: String,
    last_name: String,
}

fn meeting(s: &str) -> String {
    // makes this string uppercase
    let uppercase_s = s.to_ascii_uppercase();
    // make the string into a vector of person structs for easier sorting
    let mut people_structs: Vec<Person> = uppercase_s.split(";").map(|person_string| {
        let mut parts = person_string.split(":");
        Person {
            first_name: parts.next().unwrap().to_string(),
            last_name: parts.next().unwrap().to_string()
        }
    }).collect();
    // gives it sorted in alphabetical order by last name. When the last names are the same, sort them by first name.
    people_structs.sort_by(|a, b| {
        let last_name_ordering = a.last_name.partial_cmp(&b.last_name);
        match last_name_ordering {
            Some(Ordering::Equal) => {
                let first_name_ordering = a.first_name.partial_cmp(&b.first_name);
                first_name_ordering.unwrap()
            }
            _ => last_name_ordering.unwrap(),
        }
    });
    // Last name and first name of a guest come in the result between parentheses separated by a comma.
    people_structs.iter().fold(String::new(), |mut acc, person| {
        let pstring = format!("({}, {})", person.last_name, person.first_name);
        acc.push_str(pstring.as_str());
        acc
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        let ans = meeting(s);
        assert_eq!(ans, exp);
    }

    #[test]
    fn basic_tests() {
        dotest("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
        dotest("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
        dotest("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
            "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
    }
}
