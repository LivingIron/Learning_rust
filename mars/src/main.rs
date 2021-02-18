// getting the standart library and using the basic io
use std::io;


fn main() {
    let mut input=String::new();

// You can either create 1 mutable reference or many non mutable ones
    io::stdin().read_line(&mut input);
    println!("Input : {}", input);
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars : {}kg",mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32{
   (weight/9.81)*3.711
}
