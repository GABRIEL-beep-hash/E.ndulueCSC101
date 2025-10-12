fn main() {
    // Define what items we have and their sales data
    let products = [
        ("Toshiba", 2, 450_000.0),
        ("Mac", 1, 1_500_000.0),
        ("HP", 3, 750_000.0),
        ("Dell", 3, 2_850_000.0),
        ("Acer", 1, 250_000.0),
    ];

    // Set up variables to keep track of the total amount and item count
    let mut sum_total = 0.0;
    let mut item_count = 0;

    // Go through each product to add up the total amount and count how many items there are
    for entry in products.iter() {
        sum_total += entry.2; // The third part is the "Amount"
        item_count += entry.1; // Add the number of units sold per product
    }

    // Work out the average value of sales amounts
    let avg_value = sum_total / item_count as f64;

    // Show off our results with two decimal places displayed nicely
    println!("Total Sum: {:.2}", sum_total);
    println!("Average: {:.2}", avg_value);
}
