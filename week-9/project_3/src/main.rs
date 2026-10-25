fn main() {
    let commissioners = vec![
        "Aigbogun Alama Daudu",
        "Murtala Afeez Bendu",
        "Okorocha CalistusnOgbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defence",
        "Power & steel",
        "petroleum",
    ];

    let zones = vec![
        "south West",
        "North West",
        "south",
        "south",
        "south",
    ];

    println!("EFCC MERGED RECORD");
    println!("==================");
    
    for i in 0..commissioners.len() {
        println!(
            "commissioners: {} | Ministry: {} | zone: {}",
            commissioners[i], ministries[i], zones[i]
        );
    }
}