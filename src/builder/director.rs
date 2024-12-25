use crate::builder::Builder;

pub struct Director<'a, T: Builder> {
    builder: &'a mut T,
}

impl<'a, T: Builder> Director<'a, T> {
    pub fn new(builder: &mut T) -> Director<T> {
        Director { builder }
    }
}

impl<'a, T: Builder> Director<'a, T> {
    pub fn construct(&mut self) {
        self.builder.make_title("Greeting");
        self.builder.make_string("朝から昼にかけて");
        self.builder
            .make_items(&vec!["おはようございます。", "こんにちは。"]);
        self.builder.make_string("夜に");
        self.builder
            .make_items(&vec!["こんばんは。", "おやすみなさい。", "さようなら。"]);
        self.builder.close();
    }
}
