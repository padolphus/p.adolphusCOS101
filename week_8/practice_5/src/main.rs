use std::io;

fn main() {
    // Create an empty vector for cities
    let mut cities: Vec<String> = Vec::new();

    // Print initial length of the vector
    println!("The City vector has {} elements", cities.len());

    // Ask how many cities to enter
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    // Loop to collect city names
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}", count + 1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");

        let new_city: String = input2.trim().to_string();
        cities.push(new_city);
    }

    // Print the preferred cities
    println!("\nYour preferred cities are:");
    for (index, city) in cities.iter().enumerate() {
        println!("{} {}", index + 1, city);
    }
}

