use std::io;

fn main() {

//define the variables
 let mut experience = String::new();
 let mut age = String::new();


//enter correct option
    println!("Are you experienced?( Yes /No )");
        io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read input");


//enter age
    println!("Enter your age:");
        io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");

    let age:i32 = age.trim().parse().expect("Please a valid age");

let experience = experience.trim().to_lowercase();




if experience == "yes" && age >= 40  {
    println!(" Annual incentive:1,560,000");
}

else if experience == "yes" && age >= 30 && age <= 39 {
    println!(" Annual incentive:1,480,000");
}

else if experience == "yes" && age < 28{
    println!(" Annual incentive:1,300,000");
}

else if experience == "no" {
    println!(" Annual incentive:100,000");
}

else{
    println!("No incentives category specified for this age");
}

}
 






