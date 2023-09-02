fn main() {
    for i in (0..101).step_by(10) {
        println!("{}°C = {}°F", i, celsius_to_fahrenheit(i as f64));
    }
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
