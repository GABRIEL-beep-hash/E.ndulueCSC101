use std::collections::HashMap;

#[derive(Debug)]
struct Developer {
    name: String,
    years_experience: u32,
    programming_languages: Vec<String>,
}

fn find_most_experienced_developer(developers: &[Developer]) -> Option<&Developer> {
    developers.iter().max_by_key(|dev| dev.years_experience)
}

fn main() {
    // Using a Vector (Vec) to store multiple developers
    let mut developers = vec![
        Developer {
            name: "Alice Johnson".to_string(),
            years_experience: 7,
            programming_languages: vec!["Rust".to_string(), "Python".to_string(), "Java".to_string()],
        },
        Developer {
            name: "Bob Smith".to_string(),
            years_experience: 12,
            programming_languages: vec!["JavaScript".to_string(), "TypeScript".to_string(), "Go".to_string()],
        },
        Developer {
            name: "Carol Davis".to_string(),
            years_experience: 5,
            programming_languages: vec!["C++".to_string(), "Python".to_string(), "Rust".to_string()],
        },
        Developer {
            name: "David Wilson".to_string(),
            years_experience: 15,
            programming_languages: vec!["Java".to_string(), "Kotlin".to_string(), "Scala".to_string()],
        },
        Developer {
            name: "Eva Brown".to_string(),
            years_experience: 8,
            programming_languages: vec!["Python".to_string(), "JavaScript".to_string(), "Ruby".to_string()],
        },
    ];

    // Display all candidates
    println!("=== EY Nigeria - Developer Candidates ===");
    for (i, developer) in developers.iter().enumerate() {
        println!("{}. {} - {} years experience", i + 1, developer.name, developer.years_experience);
        println!("   Languages: {}", developer.programming_languages.join(", "));
    }

    // Find the most experienced developer
    match find_most_experienced_developer(&developers) {
        Some(most_experienced) => {
            println!("\n🎉 SELECTED CANDIDATE 🎉");
            println!("Name: {}", most_experienced.name);
            println!("Years of Experience: {}", most_experienced.years_experience);
            println!("Programming Languages: {}", most_experienced.programming_languages.join(", "));
            
            // Bonus: Using HashMap to store additional candidate metrics
            let mut candidate_metrics = HashMap::new();
            candidate_metrics.insert("experience_level", "Senior");
            candidate_metrics.insert("interview_score", "95");
            candidate_metrics.insert("status", "Selected");
            
            println!("\nCandidate Metrics:");
            for (key, value) in &candidate_metrics {
                println!("  {}: {}", key, value);
            }
        }
        None => {
            println!("No candidates found!");
        }
    }

    // Alternative approach using tuples in a vector
    println!("\n=== Alternative Approach - Experience Summary ===");
    let experience_tuples: Vec<(&str, u32)> = developers
        .iter()
        .map(|dev| (dev.name.as_str(), dev.years_experience))
        .collect();
    
    for (name, exp) in &experience_tuples {
        println!("{}: {} years", name, exp);
    }

    // Find max using the tuple approach
    if let Some((name, max_exp)) = experience_tuples.iter().max_by_key(|(_, exp)| exp) {
        println!("\nMost experienced (tuple method): {} with {} years", name, max_exp);
    }
}