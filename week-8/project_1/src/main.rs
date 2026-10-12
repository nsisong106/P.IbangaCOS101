use std::io;

fn main() {
    // Each tuple is (level name, min years, max years)
    let levels = vec![
        ("APS 1-2", 1, 2),
        ("APS 3-5", 3, 5),
        ("APS 5-8", 5, 8),
        ("EL1 8-10", 8, 10),
        ("EL2 10-13", 10, 13),
        ("SES", 13, 99),
    ];

    // Profession names, and one vector of titles per profession
    // (same order as the levels above)
    let names = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let table = vec![office_admin, academic, lawyer, teacher];

    println!("=== APS Level Checker ===");
    println!("Professions: Office Admin, Academic, Lawyer, Teacher");

    // Input 1: profession
    println!("Enter profession:");
    let mut profession_input = String::new();
    io::stdin().read_line(&mut profession_input).expect("Failed to read input");
    let profession = profession_input.trim();

    // Input 2: job title
    println!("Enter job title:");
    let mut title_input = String::new();
    io::stdin().read_line(&mut title_input).expect("Failed to read input");
    let title = title_input.trim();

    // Input 3: years of experience
    println!("Enter years of experience:");
    let mut years_input = String::new();
    io::stdin().read_line(&mut years_input).expect("Failed to read input");
    let years: i32 = match years_input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Years must be a whole number");
            return;
        }
    };

    // Step 1: find the profession
    let mut p = 99;
    for i in 0..names.len() {
        if names[i] == profession {
            p = i;
        }
    }
    if p == 99 {
        println!("Unknown profession: {}", profession);
        return;
    }

    // Step 2: find the title in that profession's row, which gives the level
    let row = &table[p];
    let mut level = 99;
    for i in 0..row.len() {
        if row[i] == title {
            level = i;
        }
    }
    if level == 99 {
        println!("'{}' is not a valid title for {}", title, profession);
        return;
    }

    // Step 3: check the years against that level's range
    let (name, min, max) = levels[level];
    if years >= min && years <= max {
        println!("VALID: {} ({}) with {} years holds {}", title, profession, years, name);
    } else {
        println!("INVALID: {} is {} and needs {} to {} years, but you entered {}", title, name, min, max, years);
    }
}