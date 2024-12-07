use crate::abstract_display::AbstractDisplay;

pub struct StringDisplay {
    string: String,
    width: usize,
}

impl StringDisplay {
    pub fn new(string: String) -> Self {
        let width = string
            .chars()
            .map(|c| if c.len_utf8() > 1 { 2 } else { 1 })
            .sum();
        Self { string, width }
    }

    fn print_line(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl AbstractDisplay for StringDisplay {
    fn open(&self) {
        self.print_line();
    }

    fn print(&self) {
        println!("|{}|", self.string);
    }

    fn close(&self) {
        self.print_line();
    }
}
