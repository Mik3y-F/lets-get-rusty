const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    let y = 6;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);

    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    shadow_example();
    floating_points_in_action();
    chars_in_action();

    let tup = lets_talk_tuples();
    println!("The value of x is: {}", tup.0);

    what_about_arrays();
    print_em_params(x, y);
    statements_vs_expressions();
    use_if_and_let();
    return_from_loops();
    loop_labels();
    iter_over_array();
    fun_for_loops();
}

fn shadow_example() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn floating_points_in_action() {
    let x = 2.01; // f64

    let y: f32 = 3.2; // f32

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn chars_in_action() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
}

pub fn lets_talk_tuples() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    return tup;
}

fn what_about_arrays() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January", "February", "March", "April", "May", "June", "July",
    ];

    println!("The value of a is: {:?}", a);
    println!("The value of months is: {:?}", months);
}

fn print_em_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statements_vs_expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn use_if_and_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn return_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);
}

fn loop_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }

    println!("Exited the outer loop");
}

fn iter_over_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value of element is: {}", element);
    }
}

fn fun_for_loops() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
