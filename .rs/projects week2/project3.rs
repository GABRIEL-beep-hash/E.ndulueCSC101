fn main() {
    let initial_value: f64 = 510000.0; // Starting price of the TV at the time is was made
    let depreciation_rate: f64 = 5.0; // Rate at which value decreases in value
    let duration_in_years: u32 = 3; // Time period in years of when bought

    // Change rate to a decimal and use  calculation
    let factor = 1.0 - (depreciation_rate / 100.0);
    let value_after_depreciation = initial_value * factor.powi(duration_in_years as i32);

    println!("After {} years, the TV's worth is ₦{:.2}", duration_in_years, value_after_depreciation);
}