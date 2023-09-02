use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let asparagus = Asparagus::new(10);

    println!("{}: {}", asparagus.name(), asparagus.quantity());
}
