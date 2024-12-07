mod abstract_display;
mod char_display;
mod string_display;

use abstract_display::AbstractDisplay;
use char_display::CharDisplay;
use string_display::StringDisplay;

fn main() {
    let d1 = CharDisplay::new('H');
    let d2 = StringDisplay::new("Hello, World.".to_string());
    let d3 = StringDisplay::new("こんにちは。".to_string());
    display(&d1);
    display(&d2);
    display(&d3);
}

fn display<T: AbstractDisplay>(d: &T) {
    d.display();
}
