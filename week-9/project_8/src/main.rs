use std::io::Write;
use std::fs::File;
fn main() {
   let lager = ["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
   let stout = ["Legend","Turbo King","Williams","","",""];
   let non_alcoholic = ["Maltina","Amstel Malta","Malta Gold","Fayrouz","",""];

let mut file = File::create("drink.txt").expect("Could not create file");
writeln!(file,"{:<15}{:<15}{:<15}","Lager", "Stout", "Non-Alcoholic").expect("Could not write to file");


for i in 0..6{
    writeln!(file,"{:<15}{:<15}{:<15}",lager[i],stout[i] ,non_alcoholic[i]).expect("Could not write to file");

}
 println!("File created succesfully!");
}