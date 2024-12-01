#[derive(Clone)]
pub struct Book {
    name: String,
}

impl Book {
    pub fn new(name: &str) -> Book {
        Book {
            name: name.to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
