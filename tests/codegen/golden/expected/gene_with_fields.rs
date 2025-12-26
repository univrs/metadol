#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub id: isize,
    pub name: String,
    pub running: bool,
}

impl Container {
    pub fn new(id: isize, name: String, running: bool) -> Self {
        Self {
            id,
            name,
            running,
        }
    }
}
