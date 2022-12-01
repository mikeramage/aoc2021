use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum CaveSize {
    LARGE,
    SMALL,
}

#[derive(Debug)]
pub struct Cave<'a> {
    name: String,
    size: CaveSize,
    connections: Vec<&'a Cave<'a>>,
}

impl PartialEq for Cave<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Cave<'_> {}

impl Hash for Cave<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> Cave<'a> {
    pub fn new(name: String) -> Cave<'a> {
        let size: CaveSize;
        if let Some(x) = &name.chars().next() {
            if x.is_lowercase() {
                size = CaveSize::SMALL;
            }
            else {
                 size = CaveSize::LARGE
            }
        }
        else {
            panic!("This cave has no name!");
        }
        Cave {
            name,
            size,
            connections: vec![], 
        }
    }

    pub fn push_connection(&mut self, cave: &'a Cave) {
        self.connections.push(cave);
    }
}