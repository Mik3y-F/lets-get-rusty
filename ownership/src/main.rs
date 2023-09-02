fn main() {
    let s1 = String::from("hello");

    let s2 = s1.clone();

    // s1.push_str(", world!"); // error
    println!("s1 = {}; s2 = {}", s1, s2);

    let s3 = s1; // move

    // println!("s1 = {}", s1); // error
    println!("s3 = {}", s3);

    takes_ownership(s3);

    // println!("s3 = {}", s3); // error

    let _ = gives_ownership();

    let s5 = String::from("hello");
    let _ = takes_and_gives_back(s5);

    print_length();
    print_first_word();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn print_length() {
    let mut s1 = String::from("hello");

    change(&mut s1);
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn print_first_word() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    // slices in action
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{} {}", i, item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
