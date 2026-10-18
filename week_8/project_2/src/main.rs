fn main() {
    // Vector of tuples: (name, years of experience)
    let candidates = vec![
        ("Alice", 3),
        ("Bob", 7),
        ("Charlie", 5),
        ("Diana", 10),
    ];

    // Start with the first candidate as "highest"
    let mut highest = candidates[0];

    // Loop through all candidates
    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    println!(
        "The person with the highest experience is {} with {} years.",
        highest.0, highest.1
    );
}
