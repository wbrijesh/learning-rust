fn main() {
    let sentence = String::from("abc def ghi jkl");

    let last_char_at = get_first_n_words(&sentence, 2);

    println!("last character is: {}", last_char_at);
    println!("the first n words are: {}", &sentence[0..last_char_at]);
}

fn get_first_n_words(str: &String, n: u8) -> usize {
    println!("getting first {} words from '{}'", n, &str);

    let bytes = str.as_bytes();
    let mut n_counter: u8 = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if n_counter == n - 1 {
                return i;
            } else {
                n_counter = n_counter + 1;
            }
        }
    }

    return str.len();
}
