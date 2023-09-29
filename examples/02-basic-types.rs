fn main() {
    // Declare a variable with each of these types: i32, u32, f64, bool, &str, and print them
    let i32v: i32 = -1;
    let u32v: u32 = 1;
    let f64v: f64 = 0.99;
    let bv: bool = true;
    let strv: &str = "banana";

    println!("int32 = {}", i32v);
    println!("unsigned32 = {}", u32v);
    println!("float64 (double) = {}", f64v);
    println!("bool = {}", bv);
    println!("string = {}", strv);
    // Declare a variable named `x` that is the sum of two integers, and print it
    let x = i32v + u32v as i32;
    println!("x {}", x);
    // Declare an array of i32 with 3 elements, and print it
    let x = [1, 2, 3];
    println!("x[1] = {}", x[1]);
    // Print the array's 2nd element

    // Make the array mutable using shadowing, and change the 2nd element to 42
    let mut x = x;
    x[1] = 42;
    println!("x[1] = {}", x[1]);
    // Declare a tuple with 3 elements: i32, &str, and f64, and print it

    // Print its 1st element
}
