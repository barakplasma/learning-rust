use std::{fmt::Display, fmt::Formatter, fmt::Result, ops::Deref};

fn main() {
    let mut s = Family::new();
    let mut f = Family::new();

    let mimi = Person::root("Mimi");
    let ed = Person::root("Ed");
    let david = Person::new("David", &mimi, &ed);
    let phillip = Person::new("Phillip", &mimi, &ed);
    let isabella = Person::new("Isabella", &mimi, &ed);
    let beatrice = Person::new("Beatrice", &mimi, &ed);
    let mia = Person::new("Mia", &mimi, &ed);
    let mary = Person::root("Mary");
    let herbert = Person::root("Herbert");
    let sari = Person::new("Sari", &mary, &herbert);
    let rob = Person::new("Rob", &mary, &herbert);

    let michael = Person::new("Michael", &sari, &david);
    let lia = Person::new("Lia", &sari, &david);

    for new_member in [
        &ed, &mimi, &david, &lia, &michael, &phillip, &isabella, &beatrice, &mia,
    ]
    .iter()
    {
        s.add_member(new_member);
    }

    for new_member in [&rob, &sari, &herbert, &mary, &lia, &michael].iter() {
        f.add_member(new_member);
    }

    michael.display_all_parents();
    println!("Siblings of {}: {}", michael, s.siblings(&michael));
    println!("Siblings of {}: {}", david, s.siblings(&david));
}

type Members<'people> = Vec<&'people Person<'people>>;
pub struct Family<'people> {
    members: Members<'people>,
}

impl<'people> Family<'people> {
    fn new() -> Family<'people> {
        Family { members: vec![] }
    }

    fn add_member(&mut self, member: &'people Person) {
        self.members.push(member);
    }

    fn siblings(&self, individual: &'people Person) -> Family {
        Family {
            members: self
                .members
                .iter()
                .filter(|m| {
                    if individual.name != m.name {
                        return match [m.dad, individual.dad] {
                            [Some(dad), Some(dad2)] => dad == dad2,
                            _ => false,
                        };
                    }
                    return false;
                })
                .map(|x| x.deref())
                .collect(),
        }
    }
}

impl Display for Family<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.members
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Person<'people> {
    name: String,
    mom: Option<&'people Person<'people>>,
    dad: Option<&'people Person<'people>>,
}

impl<'people> Person<'people> {
    fn root(name: &str) -> Person<'people> {
        Person {
            name: name.to_string(),
            mom: None,
            dad: None,
        }
    }

    fn new(name: &str, mom: &'people Person, dad: &'people Person) -> Person<'people> {
        Person {
            name: name.to_string(),
            mom: Some(mom),
            dad: Some(dad),
        }
    }

    fn display_person_at_level(&self, level: usize) {
        println!("{}{}", " ".repeat(level * 4), &self);
    }

    fn display_parent_level(&self, level: usize) {
        self.display_person_at_level(level);
        match self.dad {
            Some(d) => d.display_parent_level(level + 1),
            None => (),
        }
        match self.mom {
            Some(m) => m.display_parent_level(level + 1),
            None => (),
        }
    }

    fn display_all_parents(&self) {
        self.display_parent_level(0);
    }
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}
