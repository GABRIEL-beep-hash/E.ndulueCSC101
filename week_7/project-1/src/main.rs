// week-8/class_project_1/src/main.rs
// Class Project I - APS level checker using vectors and tuples
// Based on the table in Week 8P - Rust Vectors & Tuple (slide: Class Project I)

use std::io::{self, Write};

#[derive(Debug)]
struct Band {
    label: &'static str,          // e.g. "APS 5-8"
    years_min: Option<u8>,        // minimum years in band (None if not applicable)
    years_max: Option<u8>,        // maximum years in band
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

fn main() {
    // Table rows collected as a vector (ordered roughly as on the slide)
    let bands = vec![
        Band {
            label: "APS 1-2",
            years_min: Some(1),
            years_max: Some(2),
            office_admin: "Intern",
            academic: "-",
            lawyer: "Paralegal",
            teacher: "Placement",
        },
        Band {
            label: "APS 3-5",
            years_min: Some(3),
            years_max: Some(5),
            office_admin: "Administrator",
            academic: "Research Assistant",
            lawyer: "Junior Associate",
            teacher: "Classroom Teacher",
        },
        Band {
            label: "APS 5-8",
            years_min: Some(5),
            years_max: Some(8),
            office_admin: "Senior Administrator",
            academic: "PhD Candidate",
            lawyer: "Associate",
            teacher: "Snr Teacher",
        },
        Band {
            label: "EL1 8-10",
            years_min: Some(8),
            years_max: Some(10),
            office_admin: "Office Manager",
            academic: "Post-Doc Researcher",
            lawyer: "Senior Associate 1-2",
            teacher: "Leading Teacher",
        },
        Band {
            label: "EL2 10-13",
            years_min: Some(10),
            years_max: Some(13),
            office_admin: "Director",
            academic: "Senior Lecturer",
            lawyer: "Senior Associate 3-4",
            teacher: "Deputy Principal",
        },
        Band {
            label: "SES",
            years_min: Some(14), // interpret SES as 14+ years
            years_max: None,
            office_admin: "CEO",
            academic: "Dean",
            lawyer: "Partner",
            teacher: "Principal",
        },
    ];

    println!("Public Service APS level checker (Rust)");
    println!("Provide: Role (Office Administrator | Academic | Lawyer | Teacher), Job Title, and Years of experience\n");

    // read role
    print!("Enter role: ");
    io::stdout().flush().unwrap();
    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = normalize(&role);

    // read job title
    print!("Enter job title (e.g. Associate, Paralegal, Intern): ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = normalize(&title);

    // read years
    print!("Enter years of experience (integer): ");
    io::stdout().flush().unwrap();
    let mut years_s = String::new();
    io::stdin().read_line(&mut years_s).unwrap();
    let years: u8 = match years_s.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number for years. Exiting.");
            return;
        }
    };

    // helper to compare titles case-insensitively
    let matches_title = |cell: &str| -> bool {
        let cell_norm = normalize(cell);
        if cell_norm == "-" { return false; }
        // allow partial matches: E.g., "Senior Associate 1-2" matching "senior associate" or "associate"
        // and exact word match
        if cell_norm == title { return true; }
        // check if the provided title is contained in the cell string (e.g., "associate" in "senior associate 1-2")
        if cell_norm.contains(&title) { return true; }
        false
    };

    // map role to a selector function that returns the cell value for a given Band
    let find_band = |b: &Band| -> Option<&'static str> {
        match role.as_str() {
            "office administrator" | "office_admin" | "officeadmin" | "office" | "admin" => Some(b.office_admin),
            "academic" => Some(b.academic),
            "lawyer" => Some(b.lawyer),
            "teacher" => Some(b.teacher),
            _ => None,
        }
    };

    // search the bands for a matching title under the chosen role
    let mut found = false;
    for b in &bands {
        if let Some(cell) = find_band(b) {
            if matches_title(cell) {
                // check years fit the band
                let years_ok = match (b.years_min, b.years_max) {
                    (Some(min), Some(max)) => (years >= min) && (years <= max),
                    (Some(min), None) => years >= min,
                    (None, Some(max)) => years <= max,
                    (None, None) => true,
                };

                if years_ok {
                    println!("\n✅ VALID: '{}' with {} years matches band: {}", cell.trim(), years, b.label);
                } else {
                    // still report which band the job title belongs to and what years range is expected
                    let min_str = b.years_min.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string());
                    let max_str = b.years_max.map(|n| n.to_string()).unwrap_or_else(|| "+".to_string());
                    println!("\n⚠️  MISMATCH: '{}' belongs to {} (expected years: {}-{}), but provided {} years.",
                             cell.trim(), b.label, min_str, max_str, years);
                }
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("\nNo matching job title found under role '{}'. Double-check spelling or try another role/title.", role);
        println!("Tip: Try full role names like: Office Administrator, Academic, Lawyer, Teacher");
    }
}
