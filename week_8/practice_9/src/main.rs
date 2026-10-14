fn main() {
    // Define a tuple with explicit types
    let b: (i32, bool, f64) = (110, true, 10.9);

    // Pass the tuple to a function
    print_tuple(b);
}

// Function that accepts a tuple as a parameter
fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print function");
    println!("{:?}", x);
}
