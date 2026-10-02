use std::io;

fn main() {
    // Display the menu using \t
    println!("code\tfood\t\t\t\tprice(N)");
    println!("------------------------------------------------");
    println!("P\tPoundo Yam/Edinkaiko Soup\t3200");
    println!("F\tFried Rice & Chicken\t\t3000");
    println!("A\tAmala & Ewedu Soup\t\t2500");
    println!("E\tEba & Egusi Soup\t\t2000");
    println!("W\tWhite Rice & Stew\t\t2500");

    // Read the food code
    println!("\nEnter food code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Failed to read line");
    let code = code.trim().to_uppercase();

    // Read the quantity
    println!("Enter quantity:");
    let mut qty_text = String::new();
    io::stdin().read_line(&mut qty_text).expect("Failed to read line");
    let quantity: u32 = qty_text.trim().parse().expect("Please enter a number");

    // Find the price from the code
    let price: u32;
    if code == "P" {
        price = 3200;
    } else if code == "F" {
        price = 3000;
    } else if code == "A" {
        price = 2500;
    } else if code == "E" {
        price = 2000;
    } else if code == "W" {
        price = 2500;
    } else {
        println!("Invalid food code");
        return;
    }

    // Compute the total and discount
    let total = price * quantity;
    let mut discount = 0.0;

    if total > 10000 {
        discount = total as f64 * 0.05;
    }

    let amount_due = total as f64 - discount;

    println!("\nSubtotal:\tN{}", total);
    println!("Discount:\tN{}", discount);
    println!("Total due:\tN{}", amount_due);
}
   