fn find_nth_word(s: &str, n: usize) -> Option<String> {
    for (i, w) in s.split_whitespace().enumerate() {
        if i == n {
            return Some(String::from(w));
        }
    }
    None
}

fn main() {
    if let Some(word) = find_nth_word("Hello, world!", 1) {
        println!("Second word: {}", word);
    }

    let file = std::fs::read_to_string("data/test2.txt");
    match file {
        Ok(content) => println!("{}", content),
        Err(err) => println!("{:?}", err),
    }
}
