use std::io;
fn main() {
    println!("\nstudent information management system!");

    //input name
    println!("\nPlease Enter your name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is:{}",name);

    //input age
    println!("\nEnter your age.");
    let mut age =String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("inputnot an integer");
    println!("Your age is: {}",age);
}