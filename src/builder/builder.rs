pub trait Builder {
    fn make_title(&mut self, title: &str);
    fn make_string(&mut self, string: &str);
    fn make_items(&mut self, items: &[&str]);
    fn close(&mut self);
}
