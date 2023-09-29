fn main() {
    // Create a mutable String with `String::from(...)` and print it.
    let mut s = String::from("Hello");
    // Add ", World!" to the String and print it.
    s.push_str(", World!");
    // or
    s += ", World!";

    println!("{}", s);
}
