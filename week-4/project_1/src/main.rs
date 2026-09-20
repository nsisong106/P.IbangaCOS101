use std::io;

fn main() {


//define variables
let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();


//input for b
println!("\n enter the value of b:");
    io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");
    let b:f64 = input1.trim().parse().expect("Please enter a valid number");


//input for a
println!("\n enter the value of a:");
    io::stdin()
    .read_line(&mut input2)
    .expect("Failed to read input");
    let a:f64 = input2.trim().parse().expect("Please enter a valid number");


//input for c
println!("\n enter the value of c:");
    io::stdin()
    .read_line(&mut input3)
    .expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Please enter a valid number");


let d = b*b - 4.0 * a * c;


//when discriminant is greater than 0.0
if d > 0.0 {
  let  root1 = ( -b + d.sqrt()) /( 2.0 * a);
  let  root2 = ( -b - d.sqrt()) /( 2.0 * a);

//Root values 
println!(" Root1 is {} ", root1);
println!(" Root2 is {} ", root2);
}



//when discriminant is equal to 0.0
 else if d == 0.0 {
  
  let  root =  -b  /( 2.0 * a);

//Root value
println!(" Real root is {} ", root );
}


// when discriminant is less than 0.0
else{
    println!("No real roots");
}
    
}
