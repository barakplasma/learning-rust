fn main() {
    let mimi = Person::root("Mimi");
    let ed = Person::root("Ed");
    let david = Person::new("David", &mimi, &ed);
    let mary = Person::root("Mary");
    let herbert = Person::root("Herbert");
    let sari = Person::new("Sari", &mary, &herbert);
    let michael = Person::new("Michael", &sari, &david);
    let lia = Person::new("Lia", &sari, &david);
    lia.display_all_parents();
    michael.display_all_parents();
}

#[derive(Debug)]
pub struct Person<'people> {
    name: String,
    mom: Option<&'people Person<'people>>,
    dad: Option<&'people Person<'people>>,
}

impl<'people> Person<'people> {
    fn root(name: &str) -> Person {
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

    fn display_parents(&self, level: usize) {
        println!("{}{}", " ".repeat(level*3), &self.name);
    }

    fn display_parent_level(&self, level: usize) {
        self.display_parents(level);
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
