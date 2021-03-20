fn main() {
    let mimi = Person::root("Mimi");
    let ed = Person::root("Ed");
    let david = Person::new("David", mimi, ed);
    let mary = Person::root("Mary");
    let herbert = Person::root("Herbert");
    let sari = Person::new("Sari", mary, herbert);
    let michael = Person::new("Michael", sari, david);
    michael.display_all_parents();
}

#[derive(Debug)]
pub struct Person {
    name: String,
    mom: Option<Box<Person>>,
    dad: Option<Box<Person>>,
}

impl Person {
    fn root(name: &str) -> Person {
        Person {
            name: name.to_string(),
            mom: None,
            dad: None,
        }
    }

    fn new(name: &str, mom: Person, dad: Person) -> Person {
        Person {
            name: name.to_string(),
            mom: Some(Box::new(mom)),
            dad: Some(Box::new(dad)),
        }
    }

    fn display_parents(&self, level: usize) {
        println!("{}{}", " ".repeat(level*3), &self.name);
    }

    fn display_parent_level(&self, level: usize) {
        self.display_parents(level);
        match self.dad.as_ref() {
            Some(d) => d.display_parent_level(level + 1),
            None => (),
        }
        match self.mom.as_ref() {
            Some(m) => m.display_parent_level(level + 1),
            None => (),
        }
    }

    fn display_all_parents(&self) {
        self.display_parent_level(0);
    }
}
