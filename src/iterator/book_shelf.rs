use crate::book::Book;

pub struct BookShelf {
    books: Vec<Book>,
    index: usize,
}

impl BookShelf {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            index: 0,
        }
    }

    pub fn append_book(&mut self, book: Book) {
        self.books.push(book);
    }
}

impl Iterator for BookShelf {
    type Item = Book;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.books.len() {
            let book = self.books[self.index].clone();
            self.index += 1;
            Some(book)
        } else {
            None
        }
    }
}
