use print::Print;
use print_banner::PrintBanner;

mod banner;
mod print;
mod print_banner;

fn main() {
    let text = "Hello".to_string();
    let p = PrintBanner::new(text);
    print(&p);
}

fn print<T: Print>(p: &T) {
    p.print_weak();
    p.print_strong();
}
