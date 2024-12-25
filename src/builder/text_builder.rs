use crate::builder::Builder;

pub struct TextBuilder {
    buffer: String,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn get_result(&self) -> String {
        self.buffer.clone()
    }
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: &str) {
        self.buffer.push_str("==============================\n");
        self.buffer.push_str(format!("[ {} ]\n", title).as_str());
        self.buffer.push_str("\n");
    }

    fn make_string(&mut self, string: &str) {
        self.buffer.push_str(format!("■ {}\n", string).as_str());
        self.buffer.push_str("\n");
    }

    fn make_items(&mut self, items: &[&str]) {
        for item in items {
            self.buffer.push_str(format!(" ・{}\n", item).as_str());
        }
        self.buffer.push_str("\n");
    }

    fn close(&mut self) {
        self.buffer.push_str("==============================\n");
    }
}
