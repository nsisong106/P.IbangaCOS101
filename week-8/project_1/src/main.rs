use std::io;

//enter public service role
fn main(){
	println!(" Enter Public service:");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("failed to read input");
	let input_1 = input1.trim();

//enter  work experience 
println!(" Enter Work Experience");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("failed to read input");
	let input_2: u32 = input2.trim().parse().expect("Invalid Input");



let  office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
let academic =vec![" ","Research Assistant","PHD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
let  teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];



if input_1 == "office_admin" && input_2>=1 {
   println!("Enter Role:");
	//input a role under office_Admin
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let input_3 = input3.trim();


 if input_3 == office_admin[0]{
	println!("APS level is 1-2");
	}

   else if input_3 == office_admin[1]{
	println!("APS level is 3-5");
	}

   else if input_3 == office_admin[2]{
	println!("APS level is 5-8");
	}

   else if input_3 == office_admin[3]{
	println!("APS level is 8-10");
	}

   else if input_3 == office_admin[4]{
	println!("APS level is 10-13");
    }

   else if input_3 == office_admin[5]{
	println!("APS level is SES");
	}
	else{
		println!("Roles not recognised");
	}

  }


else if input_1 == "academic" && input_2>=1{
	println!("Enter role:");
//input a role under academic 
let mut input4 = String::new();
io::stdin().read_line(&mut input4).expect("Failed to read input");

let input_4 = input4.trim(); 

if input_4 == academic[1]{
	println!("APS level is 3-5");
	}

    else if input_4 == academic[2]{
	println!("APS level is 5-8");
	}

    else if input_4 == academic[3]{
	println!("APS level is 8-10");
	}
	else if input_4 == academic[4]{
	println!("APS level is 10-13");
	}

	else if input_4 == academic[5]{
	println!("APS level is SES");
	}
    else{
println!("Roles not recognised");
  }
}


else if input_1 == "lawyer" && input_2>=1 {
	println!("Enter role");
//input a role under lawyer 
let mut input5 = String::new();
io::stdin().read_line(&mut input5).expect("Failed to read input");
let input_5 = input5.trim();

if input_5 == lawyer[0]{
	println!("APS level is 1-2");
	}

    else if input_5 == lawyer[1]{
	println!("APS level is 3-5");
	}

    else if input_5 == lawyer[2]{
	println!("APS level is 5-8");
	}

    else if input_5 == lawyer[3]{
	println!("APS level is 8-10");
	}
	else if input_5 == lawyer[4]{
	println!("APS level is 10-13");
	}

	else if input_5 == lawyer[5]{
	println!("APS level is SES");
	}
	else{
		println!("Roles not recognized");
	}
}

else if input_1 == "teacher" && input_2>=1 {
	println!("Enter role:");
//input a role under teacher 
let mut input6 = String::new();
io::stdin().read_line(&mut input6).expect("Failed to read input");
let input_6 =input6.trim();

if input_6 == teacher[0]{
	println!("APS level is 1-2");
	}

    else if input_6 == teacher[1]{
	println!("APS level is 3-5");
	}

    else if input_6 == teacher[2]{
	println!("APS level is 5-8");
	}

    else if input_6 == teacher[3]{
	println!("APS level is 8-10");
	}


	else if input_6 == teacher[4]{
	println!("APS level is 10-13");
	}

	else if input_6 == teacher[5]{
	println!("APS level is SES");
	}

   else{
   	println!("Roles not recognized");
   }

}

}
