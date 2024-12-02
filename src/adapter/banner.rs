pub struct Banner {
    text: String,
}

impl Banner {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn show_with_paren(&self) {
        println!("({})", self.text);
    }

    pub fn show_with_aster(&self) {
        println!("*{}*", self.text);
    }
}
