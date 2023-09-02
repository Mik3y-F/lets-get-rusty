#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn _new(toast: &str, seasonal_fruit: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from(seasonal_fruit),
        }
    }

    pub fn _toast(&self) -> &String {
        &self.toast
    }

    pub fn seasonal_fruit(&self) -> &String {
        &self.seasonal_fruit
    }

    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub fn _fix_incorrect_order() {
    _cook_order();
    super::_deliver_order();
}

pub fn _cook_order() {}
