use std::io;

fn main() {

let mut input = String::new();

//input height
println!("\n  Enter your height(in centimetres):");

  io::stdin().
  read_line(&mut input)
  .expect("Not a valid string");

let height:f32 = input.trim().parse().expect("Not a valid number");



// average height
if height >= 150.0 && height <= 170.0
{
    println!(" You are average height ");   
}



// tall
else if height >170.0 && height <=195.0
{
    println!("You are tall");
}



// short
else if height <150.0 && height >100.0
{
    println!("You are dwarf");
}


    //abnormal height
    else {
        println!("abnormal height");
    }
}