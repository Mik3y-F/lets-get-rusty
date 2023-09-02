pub fn vector_me_baby() {
    let v: Vec<i32> = Vec::new();

    let t = vec![1, 2, 3];

    println!("v: {:?}", v);
    println!("t: {:?}", t);
}

pub fn vector_with_values() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v: {:?}", v);
}

pub fn vector_with_values_and_types() {
    let mut v = Vec::new();

    v.push("Hello");
    v.push("World");
    v.push("!");

    println!("v: {:?}", v);
}

pub fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // println!("does_not_exist: {:?}", does_not_exist);

    let does_not_exist = v.get(100);
    println!("does_not_exist: {:?}", does_not_exist);
}

pub fn access_after_push() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    let first = &v[0];

    println!("The first element is: {}", first);
}

pub fn iter_over_vector() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }
}

pub fn mutate_vector() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("v: {:?}", v);
}

#[derive(Debug)]
enum SpreadsheetCell {
    _Int(i32),
    _Float(f64),
    _Text(String),
}

pub fn vector_with_different_types() {
    let row = vec![
        SpreadsheetCell::_Int(3),
        SpreadsheetCell::_Text(String::from("blue")),
        SpreadsheetCell::_Float(10.12),
    ];

    println!("row: {:?}", row);
}
