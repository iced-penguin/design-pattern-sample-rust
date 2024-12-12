use framework::{Factory, Product};

mod framework;
mod idcard;

fn main() {
    let mut factory = idcard::IDCardFactory::new();
    let card1 = factory.create("Alice");
    let card2 = factory.create("Bob");
    let card3 = factory.create("Charlie");
    card1.utilize();
    card2.utilize();
    card3.utilize();
}
