    fn main() {
	// sales amount
	let sales = vec![450000.0,1500000.0,750000.0,2850000.0,250000.0];

	// calculate sum
	let sum: f64 = sales.iter().sum();

	// calculate average
	let average: f64 = sum / sales.len() as f64;

	println!("Total sales: N{:.2}", sum);
	print!("AVverage sales: N{:.2}", average);
}