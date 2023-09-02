use std::collections::HashMap;

#[derive(Debug)]
pub struct Mode {
    _value: i32,
    _count: i32,
}

impl Mode {
    fn new(value: i32, count: i32) -> Mode {
        Mode {
            _value: value,
            _count: count,
        }
    }
}

pub fn find_mode(values: &Vec<i32>) -> Mode {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in values {
        let count: &mut i32 = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut mode: i32 = 0;
    let mut count: i32 = 0;

    for (key, value) in &map {
        if *value > count {
            mode = *key;
            count = *value;
        }
    }

    Mode::new(mode, count)
}

pub fn find_median(values: &Vec<i32>) -> f64 {
    let mut sorted_values = values.clone();
    sorted_values.sort();

    let mid = sorted_values.len() / 2;

    if sorted_values.len() % 2 == 0 {
        (sorted_values[mid] + sorted_values[mid - 1]) as f64 / 2.0
    } else {
        sorted_values[mid] as f64
    }
}
