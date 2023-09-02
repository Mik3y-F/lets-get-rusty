use std::fs::File;

pub fn _read_file() {
    let f = File::open("hello.txt");

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

pub fn _read_file_with_unwrap() {
    let _ = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn _read_file_with_expect() {
    // The expect method is similar to unwrap, but it lets us also choose the panic! error message.
    // Used to expect that a function call will succeed instead of calling unwrap and panicking on an error.
    let _ = File::open("hello.txt").expect("Failed to open hello.txt");
}
