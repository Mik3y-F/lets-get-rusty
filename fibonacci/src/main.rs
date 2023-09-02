use std::io;

fn main() {
    println!("Enter the nth fibonacci number you want to calculate: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");
    println!(
        "The nth [{}] fibonacci number is {}",
        input,
        fibonacci(input)
    );
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
