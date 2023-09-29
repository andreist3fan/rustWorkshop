use core::num;

// 1. Write a function that takes ownership of a String, then counts the number of words in the String and returns the number of words.
fn take_ownership_and_count(s: String) -> usize {
    let count: usize = s.split_whitespace().count();
    count
}

// 3. Write a function that takes ownership of a String, splits it into words and returns a tuple with the String and the number of words.
fn take_ownership_count_then_give_back(s: String) -> (String, usize) {
    let count: usize = s.split_whitespace().count();
    (s, count)
}

// 5. Write a function that takes a reference to a String, splits it into words and returns the number of words.
fn take_reference_and_count(s: &String) -> usize {
    s.split_whitespace().count()
}

fn main() {
    let s = String::from("Hello there crab");

    // 2. Clone `s` and count words with `take_ownership_and_count()`
    let num_words = take_ownership_and_count(s.clone());
    println!("{} has {} words", s, num_words);

    // 4. Count words with `take_ownership_and_count_then_give_back()`
    let (s, num_words) = take_ownership_count_then_give_back(s);
    println!("{} has {} words", s, num_words);

    // Create a scope with a String, the create a reference to it and try to return it from the scope
    let s_ref = {
        let s = String::from("Hello there crab");
        let s_ref = &s; // error: `s` does not live long enough
                        // Uncomment and see what happens
                        // s_ref
    };

    // 6. Count words with `take_reference_and_count()`
    let num_words = take_reference_and_count(&s);
    println!("{} has {} words", s, num_words);

    // 8. Call `remove_first_word()` with a mutable reference to `s`
    // remove_first_word(...);
    // println!("with first word removed: {}", s);
}

// 7. Uncomment the function and try to make it work
// fn remove_first_word(/* ... */) {
//     // Ignore implementation details
//     let words: Vec<&str> = s.split_whitespace().collect();
//     if let Some(first_word) = words.first() {
//         let start_index = s.find(first_word).unwrap();
//         s.drain(..start_index + first_word.len());
//     }
// }
