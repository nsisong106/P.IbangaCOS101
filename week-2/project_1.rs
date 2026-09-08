fn main(){
	let p = 520000000 ;
	let r = 10 ;
	let n = 5 ;

	let a = p*[1+(r/100 )]^n ;

	println!( " amount is {}", a);

	let ci = a-p;

	println!(" compound interest is {}",ci);
}