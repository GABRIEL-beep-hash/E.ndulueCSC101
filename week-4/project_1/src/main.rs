// rust program to find the root of a quadractic equation

use std::io;
 
fn  main() {
    let mut input1= String::new();
    let mut input2= String::new();
    let mut input3= String::new();

    println!("Enter first digit: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    
    println!("Enter second digit: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b:f32= input2.trim().parse().expect("Not a valid number");


    println!("Enter third digit: ");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let discriminant:f32 = b * b - 4.0 * a * c;

    //for positive
    let root1 = (-b + discriminant.sqrt()) /(2.0*a);

    //for negative
    let root2 = (-b - discriminant.sqrt()) /(2.0*a);

    if discriminant > 0.0 {
        println!("Discriminant is positive → two real root");

    } else if discriminant == 0.0 {
        println!("Discriminant is zero → one real root");  

    } else {
        println!("Discriminant is negative →complex root")

    }
 

    println!("Root 1 ={}",root1);
    println!("Root 2 ={}",root2);
    }