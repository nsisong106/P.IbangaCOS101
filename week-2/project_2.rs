fn main(){
   // for Toshiba
	let toshiba:f64 = 450000.00;
	let toshibaQty = 2;
	let toshibaTtl = toshiba * toshibaQty;

//for Mac
	let mac:f64 = 1500000.00;
 let macQty = 1;
	let macTtl = mac * macQty;

//for Hp
	let hp:f64 = 750000.00;
	let hpQty = 3;
	let hpTtl = hp * hpQty;

//for Dell
	let dell:f64 = 2850000.00;
 let dellQty = 3;
	let dellTtl = dell * dellQty;

	//for acer
	let acer:f64 = 250000.00;
 let acerQty = 1;
	let acerTtl = acer * acerQty;


// sum of sales record
	let total = toshibaTtl + macTtl + hpTtl + dellTtl + acerTtl ;
	println!("the sum is {}", total );

let ave = total /toshibaQty + macQty + hpQty + dellQty + acerQty ;
println!("the average is {}", ave );
} 