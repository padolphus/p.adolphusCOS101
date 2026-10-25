use std::io::Write;

fn main() {
     let students = vec![
    "Student Name,Matric Number,Department,Level",
    "Oluchi Mordi,ACC10211111,Accounting,300",
    "Adams Aliyu,ECO10110101,Economics,100",
    "Shalia Balade,CSC10328827,Computer,200",
    "Adekunle Gold,EEE1020202,Electircal,200",
    "Blamca Edemoh,MEE102001,Mechanical,100",
    ];
let mut file = std::fs::File::create("PAU-SIMS.csv").expect("cannot create file");
for row in students{
    println!("{}",row);
    let line = format! ("{}\n",row);
    file.write_all(line.as_bytes()).expect("failed to write row")
}
println!("\nPAU-SIMS is saved");

}
