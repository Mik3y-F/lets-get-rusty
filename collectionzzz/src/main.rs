mod hash_map;
mod mode;
mod pig_latin;
mod strings;
mod vectors;

fn main() {
    println!("Hello, world!");

    vectors::vector_me_baby();
    vectors::vector_with_values();
    vectors::vector_with_values_and_types();
    vectors::read_vector();
    vectors::access_after_push();
    vectors::iter_over_vector();
    vectors::mutate_vector();
    vectors::vector_with_different_types();

    strings::string_soup();

    hash_map::hash_map();

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9];

    let mode = mode::find_mode(&values);
    println!("mode: {:?}", mode);

    let median = mode::find_median(&values);
    println!("median: {:?}", median);
}
