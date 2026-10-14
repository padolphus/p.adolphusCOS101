fn main() {
    // Mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // Mutable slice (from index 1 to 3 exclusive)
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // Change the value of the slice at index 1 (which corresponds to "yellow")
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}

