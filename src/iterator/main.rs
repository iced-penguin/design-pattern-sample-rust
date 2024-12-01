use book::Book;
use book_shelf::BookShelf;

mod book;
mod book_shelf;

fn main() {
    let mut book_shelf = BookShelf::new();
    book_shelf.append_book(Book::new("Around the World in 80 Days"));
    book_shelf.append_book(Book::new("Bible"));
    book_shelf.append_book(Book::new("Cinderella"));
    book_shelf.append_book(Book::new("Daddy-Long-Legs"));
    for book in book_shelf {
        println!("{}", book.get_name());
    }
}
