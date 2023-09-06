fn get_first_word(sentence: &String, n: usize) -> &str {
    let bytes = sentence.as_bytes();
    let mut word_count: usize = 0;


    for (i, &character) in bytes.iter().enumerate() {
        if character.is_ascii_whitespace() {
            if word_count == n - 1 {
                return &sentence[0..i];
            } else {
                word_count += 1;
            }
        }
    }

    return sentence.as_str();
}

fn main() {
    let example_sentence = String::from("lorem ipsum dolor sit amet");
    println!("{}", get_first_word(&example_sentence, 4));
}
