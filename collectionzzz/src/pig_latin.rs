fn _convert_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if _is_vowel(first_char) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", chars.as_str(), first_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_pig_latin() {
        assert_eq!(_convert_to_pig_latin("first"), "irst-fay");
        assert_eq!(_convert_to_pig_latin("apple"), "apple-hay");
    }
}

fn _is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
