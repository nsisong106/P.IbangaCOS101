use std::fs::File;
use std::io::Write;

fn main() {
    let names = ["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
     let matric = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
   let dept = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
let level = [300,100,200,200,100];

let mut file = File::create("students.txt").expect("Could not create file");

 // Header: screen, then file
    println!("{:<20}{:<18}{:<16}{:<8}", "Student Name", "Matric. Number", "Department", "Level");
    writeln!(file, "{:<20}{:<18}{:<16}{:<8}", "Student Name", "Matric. Number", "Department", "Level")
        .expect("Could not write to file");

    // Rows: screen, then file
    for i in 0..5 {
        println!("{:<20}{:<18}{:<16}{:<8}", names[i], matric[i], dept[i], level[i]);
        writeln!(file, "{:<20}{:<18}{:<16}{:<8}", names[i], matric[i], dept[i], level[i])
            .expect("Could not write to file");
    }
}