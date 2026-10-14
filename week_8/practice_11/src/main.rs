fn main() {
    // An array of numbers
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // Create a slice of 2nd and 3rd elements
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    // Omit the start index (slice from index 0 up to 3 exclusive)
    let slice2 = &numbers[..3];
    println!("Index 0 to index 3 sliced = {:?}", slice2);

    // Omit the end index (slice from index 2 up to the end)
    let slice3 = &numbers[2..];
    println!("Index 2 to index 5 sliced = {:?}", slice3);

    // Omit both start and end index (reference the whole array)
    let slice4 = &numbers[..];
    println!("Index 0 to index 5 sliced = {:?}", slice4);
}

