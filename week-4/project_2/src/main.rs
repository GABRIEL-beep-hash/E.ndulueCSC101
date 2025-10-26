use std::io;


 fn main () {
let mut input = String::new();

println!("age of employee");
io::stdin().read_line(&mut input).expect("Not a valid String");
let  age:f32= input.trim().parse().expect("Not a valid age");

if age >=40.0 { 
    println!("1,570,000"); }
else if age >= 30.0 && age< 40.0
        {println!("1,480,000")}
else if age == 28.0 {
     println!("1,300,000");
}    
else { 
    println!("Your salary is 100,000 (employee is inexperienced)");

}

}