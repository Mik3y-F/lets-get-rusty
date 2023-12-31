pub use crate::front_of_house::hosting; // Re-exporting

mod back_of_house;
mod front_of_house; // Relative path

mod customer {
    use super::hosting;

    pub fn _order() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!(
        "I'd like {} toast please - and a side of {:?}",
        meal.toast,
        meal.seasonal_fruit()
    );

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order 1: {:?}", order1);
    println!("Order 2: {:?}", order2);
}

fn _deliver_order() {}
