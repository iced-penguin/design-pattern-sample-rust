use crate::{banner::Banner, print::Print};

pub struct PrintBanner {
    banner: Banner,
}

impl PrintBanner {
    pub fn new(text: String) -> Self {
        Self {
            banner: Banner::new(text),
        }
    }
}

impl Print for PrintBanner {
    fn print_weak(&self) {
        self.banner.show_with_paren();
    }

    fn print_strong(&self) {
        self.banner.show_with_aster();
    }
}
