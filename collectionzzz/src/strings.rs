pub fn string_soup() {
    let mut s = String::new();
    s.push_str("Hello, ");

    println!("s: {}", s);

    let s = "initial contents".to_string();

    println!("s: {}", s);

    concatenate_strings();
}

pub fn concatenate_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2;

    println!("s3: {}, s2: {}", s3, s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s: {}", s);
}
