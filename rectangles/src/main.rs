mod shapes;

use shapes::Rectangle;

fn main() {
    let rect1 = Rectangle::new(30, 50).expect("Invalid dimensions");

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    dbg!(&rect1);

    let rect2 = Rectangle::new(10, 20).expect("Invalid dimensions");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(10);
    dbg!(&square);
}
