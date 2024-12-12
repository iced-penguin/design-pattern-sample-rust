use crate::framework::{Factory, Product};

pub struct IDCard {
    owner: String,
}

impl IDCard {
    pub fn new(owner: &str) -> Self {
        println!("{}のカードを作ります。", owner);
        Self {
            owner: owner.to_string(),
        }
    }
}

impl Product for IDCard {
    fn utilize(&self) {
        println!("{}のカードを使います。", self.owner);
    }
}
pub struct IDCardFactory {
    owners: Vec<String>,
}

impl IDCardFactory {
    pub fn new() -> Self {
        Self { owners: Vec::new() }
    }
}

impl Factory for IDCardFactory {
    type Product = IDCard;

    fn create_product(&self, owner: &str) -> Self::Product {
        IDCard::new(owner)
    }

    fn register_product(&mut self, product: &Self::Product) {
        self.owners.push(product.owner.clone());
    }
}
