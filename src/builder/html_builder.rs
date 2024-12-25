use crate::builder::Builder;
use std::fs::File;
use std::io::{BufWriter, Write};
pub struct HTMLBuilder {
    writer: BufWriter<File>,
    filename: String,
}

impl HTMLBuilder {
    pub fn new(filename: &str) -> Self {
        Self {
            writer: BufWriter::new(File::create(filename).unwrap()),
            filename: String::from(filename),
        }
    }

    pub fn get_result(&self) -> String {
        self.filename.clone()
    }
}

impl Builder for HTMLBuilder {
    fn make_title(&mut self, title: &str) {
        let s = format!(
            "<html><head><title>{}</title></head><body>\n<h1>{}</h1>\n",
            title, title
        );
        self.writer.write(s.as_bytes()).unwrap();
    }

    fn make_string(&mut self, string: &str) {
        let s = format!("<p>{}</p>\n", string);
        self.writer.write(s.as_bytes()).unwrap();
    }

    fn make_items(&mut self, items: &[&str]) {
        let mut s = String::from("<ul>\n");
        for item in items {
            s.push_str(format!("<li>{}</li>\n", item).as_str());
        }
        s.push_str("</ul>\n");
        self.writer.write(s.as_bytes()).unwrap();
    }

    fn close(&mut self) {
        self.writer.write("</body></html>\n".as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }
}
