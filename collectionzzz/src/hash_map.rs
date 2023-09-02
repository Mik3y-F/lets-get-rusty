use std::collections::HashMap;

pub fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    add_key_value();
    find_mode();
}

fn add_key_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores: {:?}", scores);
    find_word_instances();
}

fn find_word_instances() {
    let text: &str = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map: {:?}", map);
}

fn find_mode() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 1, 1, 1];

    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in &v {
        let count: &mut i32 = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut mode: i32 = 0;
    let mut mode_count: i32 = 0;

    for (key, value) in &map {
        if *value > mode_count {
            mode = *key;
            mode_count = *value;
        }
    }

    println!("mode: {}", mode);
}
