pub trait Product {
    fn utilize(&self);
}
pub trait Factory {
    type Product;

    fn create(&mut self, owner: &str) -> Self::Product {
        let p = self.create_product(owner);
        self.register_product(&p);
        p
    }
    fn create_product(&self, owner: &str) -> Self::Product;
    fn register_product(&mut self, product: &Self::Product);
}
