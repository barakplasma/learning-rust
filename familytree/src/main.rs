fn main() {
    let mut s = Family::new();
    let mut f = Family::new();

    let mimi = Person::root("Mimi");
    let ed = Person::root("Ed");
    let david = Person::new("David", &mimi, &ed);
    let mary = Person::root("Mary");
    let herbert = Person::root("Herbert");
    let sari = Person::new("Sari", &mary, &herbert);
    let rob = Person::new("Rob", &mary, &herbert);

    let michael = Person::new("Michael", &sari, &david);
    let lia = Person::new("Lia", &sari, &david);

    for new_member in [&ed, &mimi, &david, &lia, &michael].iter() {
        s.add_member(new_member);
    }

    for new_member in [&rob, &sari, &herbert, &mary, &lia, &michael].iter() {
        f.add_member(new_member);
    }

    // michael.display_all_parents();
    s.siblings(&michael);
    f.siblings(&sari);
    s.siblings(&ed);
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

    fn siblings(&self, individual: &'people Person) {
        println!("Siblings of {}: ", individual.name);
        self.members.iter().filter(|m| {
            match [m.dad, individual.dad] {
                [Some(dad), Some(dad2)] => dad == dad2,
                _ => false
            }
        }).for_each(|sibling| print!("{}, ", sibling.name));
        println!("");
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
        println!("{}{}", " ".repeat(level*3), &self.name);
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
