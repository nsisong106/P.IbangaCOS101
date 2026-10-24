use std::io::Write ;
use std::fs::File;


//define arrays
fn main() {
 let commissioner =["Aigbogun Alamba Dauda","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
 let ministry = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
let zone = ["South West","North West","South South","South West","South East"];

//create file 
let mut file = File::create("EFFC merge.txt").expect("Could not create file");
 let line = format!("{:<15}{:<15}{:<15}\n","Commissioner","Ministry","Zone");
file.write_all(line.as_bytes()).expect("Could not write to file");


//iterate array values
for i in 0..5 {
    println!("{:<35}{:<35}{:<35}\n",commissioner[i],ministry[i],zone[i]);
    let _data = format!("{:<35}{:<35}{:<35}",commissioner[i],ministry[i],zone[i]);
    file.write_all(_data.as_bytes()).expect("Could not write to file");

}
println!("File Recovered")
}
