use std::io;

fn main() {
    println!("MENU");
        //menu for the food for customers
 println!("P=poundo yam/Edinkaiko soup-N3200");
 println!("F=fried rice & chicken-N3000");
 println!("A=Amala & ewedu soup-N2500"); 
 println!("E=Eba & egusi soup-N2000");
 println!("W=white rice & stew-N2500");

println!("enter your food type (P/F/A/E/W)"); 
let mut food=String::new();
io::stdin().read_line(&mut food).expect("Wrong entry");
let food = food.trim().to_uppercase();


println!("enter quantity");
let mut qty =String::new();
io::stdin().read_line(&mut qty).expect("Not a valid ");
let qty:i32 = qty.trim().parse().expect("Not a valid quantity");

let price = match food.as_str() {
    "P" => 3200,
    "F" => 3000,
    "A" => 2500,
    "E" => 2000,
    "W" => 2500, 
  _ => { println!("price");
        return;
    }
};
let mut total = price * qty;
  if total > 10_000 {
let discount = total as f32 * 0.05;    
total -= discount as i32;
 println!("5% discount applied!");
}
println!("Total charge:N{}", total);
}
        

