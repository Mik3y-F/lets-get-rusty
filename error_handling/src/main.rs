mod error_propagation;
mod recoverable_errors;
mod unrecoverable_errors;

fn main() {
    // unrecoverable_errors::access_out_of_scope_variable();
    // recoverable_errors::read_file();
    // recoverable_errors::read_file_with_unwrap();
    // recoverable_errors::read_file_with_expect();

    error_propagation::read_username_from_file().expect("Failed to read username from file");
    error_propagation::read_username_from_file_with_question_mark_operator().unwrap_or_else(
        |error| {
            println!("Failed to read username from file: {}", error);
            String::from("") // Return a String value here
        },
    );
}
