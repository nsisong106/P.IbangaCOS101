fn main() {
    //Define variables
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str ="Km 52 Lekki-Epe Expressway,Ibeju-lekki,Lagos";

println!("Name: {}", name);
println!(" University: {}, \n Address:{} ",uni, addr);

//Define variables 
let department:&'static str = "Computer Science";
let school:&'static str = "School of Science and Technology";

println!("department:{}, \n School:{}", department ,school);
}
    
