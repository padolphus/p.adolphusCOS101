use std::io;

fn trapezium_area(base1: f64, base2: f64, height: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

fn main() {
    println!("Choose a shape: trapezium, rhombus, parallelogram, cube, cylinder");

    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("Failed to read input");
    let shape = shape.trim().to_lowercase();

    match shape.as_str() {
        "trapezium" => {
            let base1 = read_number("Enter base1: ");
            let base2 = read_number("Enter base2: ");
            let height = read_number("Enter height: ");
            println!("Area of trapezium: {}", trapezium_area(base1, base2, height));
        }
        "rhombus" => {
            let d1 = read_number("Enter diagonal1: ");
            let d2 = read_number("Enter diagonal2: ");
            println!("Area of rhombus: {}", rhombus_area(d1, d2));
        }
        "parallelogram" => {
            let base = read_number("Enter base: ");
            let altitude = read_number("Enter altitude: ");
            println!("Area of parallelogram: {}", parallelogram_area(base, altitude));
        }
        "cube" => {
            let side = read_number("Enter side length: ");
            println!("Surface area of cube: {}", cube_surface_area(side));
        }
        "cylinder" => {
            let radius = read_number("Enter radius: ");
            let height = read_number("Enter height: ");
            println!("Volume of cylinder: {}", cylinder_volume(radius, height));
        }
        _ => println!("Unknown shape!"),
    }
}

