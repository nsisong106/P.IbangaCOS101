 fn main(){
 	let p =1000;
 	let r:= 1;
 	let t = 2;

 	// simple interest
 	let a = p * (1.0 +(r / 100.0)) * t;
 	println!(" amount is {}", a);
 	let si = a-p ;
 	println!( "simple interest is {}", si);
 }