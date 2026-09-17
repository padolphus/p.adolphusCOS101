use std::io;

fn main() {
    println!("Is the employee experienced? (yes/no): ");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience = experience_input.trim().to_lowercase();

    println!("Enter employee age: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age <= 39 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            // Handle ages 28–29 (not explicitly listed, but we can assign same as below 28)
            1_300_000
        }
    } else {
        100_000
    };

    println!("Annual incentive: ₦{}", incentive);
}


