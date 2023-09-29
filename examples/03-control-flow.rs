fn main() {
    // Declare a variable named `x`
    let x = 3;
    // Create a scope and declare a variable named `y` in it. Print `x + y` from the scope.
    let z = {
        let y = 1;
        x + y
    };
    println!("{}", z);
    // Create a scope that returns `x + y` from it, assign it to a variable named `z` and print it.

    // Use if/else to print whether z is greater than 1 or not.

    // Use a `loop` to print `z` 5 times.
    let mut i = 0;
    loop {
        println!("z={}", z);
        i += 1;
        if i > 5 {
            break;
        }
    }
    // Use a while loop to print `z` 5 times (use an additional counter variable).
    let mut i = 0;
    while i < 5 {
        // ...
    }
    // Use if else to declare a variable that is true if `z` is greater than 1 and false otherwise.
    /*let z = {
        if (x > 5) {
            true
        } else {
            false
        }
    };*/
    // Use a loop that returns the first randomly generated value that is greater than 0.5 (use the function `rand::random::<f64>()` to get a random float between 0 and 1).
    let z = loop {
        let r = rand::random::<f64>();
        if (r > 0.5) {
            break r;
        }
    };

    // Use a for loop to print the numbers 1 to 10.
    for i in 1..10 {
        print!("{} ", i);
    }
    // Use a for loop to iterate throuth and array
    let arr = [1, 2, 3];
    for i in arr.iter() {
        print!("{} ", i);
    }
}
