use std::fs;

fn main() {
   fs::remove_file("data.txt").expect("Could mot remove file");
   println!("file is removed");
}