use std::io;

fn trapezium() {
    let mut input = String::new();

    println!("Enter height, base1 and base2:");
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let height = values[0];
    let base1 = values[1];
    let base2 = values[2];

    let area = height / 2.0 * (base1 + base2);

    println!("Area of trapezium = {}", area);
}

fn rhombus() {
    let mut input = String::new();

    println!("Enter diagonal1 and diagonal2:");
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let area = values[0] * values[1] / 2.0;

    println!("Area of rhombus = {}", area);
}

fn parallelogram() {
    let mut input = String::new();

    println!("Enter base and altitude:");
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let area = values[0] * values[1];

    println!("Area of parallelogram = {}", area);
}

fn cube() {
    let mut input = String::new();

    println!("Enter side of cube:");
    io::stdin().read_line(&mut input).unwrap();

    let side: f64 = input.trim().parse().unwrap();

    let area = 6.0 * side * side;

    println!("Surface area of cube = {}", area);
}

fn cylinder() {
    let mut input = String::new();

    println!("Enter radius and height:");
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let pi = 3.142;
    let area = pi * values[0] * values[0] * values[1];

    println!("Cylinder result = {}", area);
}

fn main() {
    println!("SHAPE CALCULATOR");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => trapezium(),
        "2" => rhombus(),
        "3" => parallelogram(),
        "4" => cube(),
        "5" => cylinder(),
        _ => println!("Invalid choice!"),
    }
}