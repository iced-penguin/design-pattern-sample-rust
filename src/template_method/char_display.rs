use crate::abstract_display::AbstractDisplay;

pub struct CharDisplay {
    ch: char,
}

impl CharDisplay {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}

impl AbstractDisplay for CharDisplay {
    fn open(&self) {
        print!("<<");
    }

    fn print(&self) {
        print!("{}", self.ch);
    }

    fn close(&self) {
        println!(">>");
    }
}
