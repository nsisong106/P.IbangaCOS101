use std::io;

fn main() {

   // input lower bound
    println!("Enter lower bound");
let mut input1 = String::new();

  io::stdin()
  .read_line(&mut input1)
  .expect("Failed to read input");
let lower_bound:u32 = input1.trim().parse().expect("Failed to input");




  println!("Enter upper bound");
let mut input2 = String::new();

  io::stdin()
  .read_line(&mut input2)
  .expect("Failed to read input");
let upper_bound:u32 = input2.trim().parse().expect("Failed to input");



//using for loop
for x in lower_bound..upper_bound{
    //upper bound is not inclusive
   println!("Count level is {}" , x);
}

}
