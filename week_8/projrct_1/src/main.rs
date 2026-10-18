#[derive(Debug)]
enum APSLevel {
    APS1_2,
    APS3_5,
    APS5_8,
    EL1_8_10,
    EL2_10_13,
    SES,
}

#[derive(Debug)]
struct StudentRole {
    title: &'static str,
    level: APSLevel,
}

fn main() {
    // Define student/academic progression
    let student_roles = vec![
        StudentRole { title: "Research Assistant", level: APSLevel::APS3_5 },
        StudentRole { title: "PhD Candidate", level: APSLevel::APS5_8 },
        StudentRole { title: "Post-Doc Researcher", level: APSLevel::EL1_8_10 },
        StudentRole { title: "Senior Lecturer", level: APSLevel::EL2_10_13 },
        StudentRole { title: "Dean", level: APSLevel::SES },
    ];

    // Example: Validate a PhD Candidate
    let query_title = "PhD Candidate";

    if let Some(role) = student_roles.iter().find(|r| r.title == query_title) {
        println!("{} is at level {:?}", role.title, role.level);
    } else {
        println!("Role not found!");
    }
}

