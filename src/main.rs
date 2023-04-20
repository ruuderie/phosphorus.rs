use std::io;

fn main() {
    // Enter the amount of the nutrient compound needed in grams
    let mut input = String::new();
    println!("Enter the amount of nutrient compound needed in grams:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let amount: f32 = input.trim().parse().expect("Invalid input.");

    // Calculate the amount of the nutrient compound in waste
    let waste_n = 11.0; // nitrogen content in waste in grams per liter
    let waste_p = 1.4; // phosphorus content in waste in grams per liter
    let waste_k = 2.5; // potassium content in waste in grams per liter
    let waste_volume_n = amount / waste_n; // calculate the volume of waste needed for nitrogen in liters
    let waste_volume_p = amount / waste_p; // calculate the volume of waste needed for phosphorus in liters
    let waste_volume_k = amount / waste_k; // calculate the volume of waste needed for potassium in liters
    
    // Print the result
    println!("For {} grams of the nutrient compound, you would need approximately {:.2} liters of waste for nitrogen, {:.2} liters of waste for phosphorus, and {:.2} liters of waste for potassium.", amount, waste_volume_n, waste_volume_p, waste_volume_k);
    
    // Calculate the number of people needed to produce the required amount of waste
    let waste_per_week = 10.5; // average waste volume per person per week in liters
    let people_needed_n = (waste_volume_n / waste_per_week).ceil() as i32; // round up to the nearest integer
    let people_needed_p = (waste_volume_p / waste_per_week).ceil() as i32; // round up to the nearest integer
    let people_needed_k = (waste_volume_k / waste_per_week).ceil() as i32; // round up to the nearest integer
    
    // Print the result
    println!("To produce this amount of nitrogen, you would need {} to {} people producing 7 to 14 liters of waste per week.", people_needed_n, 2 * people_needed_n);
    println!("To produce this amount of phosphorus, you would need {} to {} people producing 7 to 14 liters of waste per week.", people_needed_p, 2 * people_needed_p);
    println!("To produce this amount of potassium, you would need {} to {} people producing 7 to 14 liters of waste per week.", people_needed_k, 2 * people_needed_k);
}
