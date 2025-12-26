#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub r#type: i64,
}

impl Counter {
    pub fn new(r#type: i64) -> Self {
        Self {
            r#type,
        }
    }
}
