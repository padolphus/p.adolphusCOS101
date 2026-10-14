fn main() {
    // Define a tuple with explicit types
    let b: (i32, bool, f64) = (30, true, 4.9);

    // Pass the tuple to a function
    print_tuple(b);
}

// Function that accepts a tuple and destructures it
fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print function");

    // Assign tuple elements to distinct variables
    let (age, is_male, cgpa) = x;

    println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}

