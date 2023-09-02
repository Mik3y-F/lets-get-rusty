#[derive(Debug)]
pub struct Asparagus {
    name: String,
    quantity: u32,
}

impl Asparagus {
    pub fn new(quantity: u32) -> Self {
        Self {
            name: String::from("Asparagus"),
            quantity,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}
