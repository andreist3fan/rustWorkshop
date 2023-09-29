fn main() {
    // Declare a variable
    let x: i32 = 1; // immutable

    // Print its value
    println!("x = {}", x);
    println!("Hello world!");
    // Try changing the variable
    /*
    x = 2; // -> cannot assign twice to immutable variable `x`
    cannot assign twice to immutable variable
    */
    // Make it mutable
    let mut y = 3;
    // Try changing the variable again
    println!("y = {}", y);

    y = 75;

    println!("y = {}", y);
    // Try shadowing the variable

    let mut x = 7;
    x = 8;
    println!("x = {}", x);

    // the other x declared previously is inaccessible
}
