// getting the standart library and using the basic io
use std::io;


fn main() {

// you need to make a variable mutable before you can change the value
    println!("Enter your weight (kg): ");
    let mut input=String::new();

    io::stdin().read_line(&mut input).unwrap();
    let weight: f32=input.trim().parse().unwrap();
    
   // borrow_string(&input);
   // own_string(input);
// You can either create 1 mutable reference or many non mutable ones
   
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars : {}kg",mars_weight);
    
}

fn calculate_weight_on_mars(weight: f32) -> f32{
   (weight/9.81)*3.711
}

fn borrow_string(s: &String){
    println!("{}",s);
}

//when passing reference we just create a pointer to the value instead of granting ownership of the value to s

fn own_string(s: String){
    println!("{}",s);
}


//1. Each value in rust is owned by a variable

//2. When owner gets out of scope value will be deallocated

//3. There can only be ONE owner at at time 

// Stack is the memory allocated known at compile time , heap not known on compile time  , such as input string is type of a smart pointer

// let a=5 ; b=a; is valid since size is known at compile time but not valid for strings 

// nqma vzimane po stoinost  by default vzima ownership na podadenata stoinost , can be prevented by passing it as a reference a.k.a borrowing
// slaga se & predi imeto za da se podade po reference , a ako shte se promenq trqbwa da e &mut

/*======================================================================================== */
//let s1=&input;
//let s1=&input;
//you can borrow a value / get a non-mutable reference on whoever many varaibles you want or you can borrow mutably on 1 variable


//let mut s1=&mut input;
//let s1=&input;
//invalid because you have a non mutable borrow and a mutable borrow

//let mut s1=&mut input;
//let mut s2=&mut input;
//invalid again because you can only have 1 mutable borrow 

//let mut input=String::new();
//let s1=&input;
//let s1=&input;
//println!("{} {}",s1,s2);
//somefn(&mut input);
//valid because compiler sees that the last use of the s1 and s2 are before the call of somefn with the mutable reference where it could be changed