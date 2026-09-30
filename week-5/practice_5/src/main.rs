fn main() {

    let fullname ="Pan-Atlantic University";
    
println!();
println!("Name:{}", fullname );
println!();

//before trim
println!("Before trim");
println!("Length is {}",fullname.len());
println!();

//after trim
println!( "After trim");
println!("Length is {}", fullname.trim().len());

}
