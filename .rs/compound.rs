use std::io;

fn main() {
    println!("=== Compound Interest Calculator ===");

    // Input principal
    let mut p_input = String::new();
    println!("Enter the Principal amount (₦): ");
    io::stdin().read_line(&mut p_input).expect("Failed to read input");
    let p: f64 = p_input.trim().parse().expect("Please enter a valid number");

    // Input rate
    let mut r_input = String::new();
    println!("Enter the Annual Interest Rate (%): ");
    io::stdin().read_line(&mut r_input).expect("Failed to read input");
    let r: f64 = r_input.trim().parse().expect("Please enter a valid number");

    // Input time
    let mut n_input = String::new();
    println!("Enter the Time in years: ");
    io::stdin().read_line(&mut n_input).expect("Failed to read input");
    let n: f64 = n_input.trim().parse().expect("Please enter a valid number");

    // Calculate amount and compound interest
    let a = p * (1.0 + r / 100.0).powf(n);
    let ci = a - p;

    // Display results
    println!("\n=== Calculation Result ===");
    println!("Principal (P): ₦{:.2}", p);
    println!("Rate (R): {:.2}% per annum", r);
    println!("Time (n): {:.0} years", n);
    println!("Amount (A): ₦{:.2}", a);
    println!("Compound Interest (CI): ₦{:.2}", ci);
}

