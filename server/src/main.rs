fn main() {
    /*let string = String::from("127.0.0.1:8080");
    let string_slice= &string[10..]; // 10..14 syntax for specifying ranges in rust what it does is gets all the characters from the 10th byte to the 14th one also 10.. or ..3
    let string_borrow: &str =&string;   //&str - string slice is just a pointer within a string so it does not allocate a new string in the memory 
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    */
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server{
    addr:String ,
}

impl Server{

    fn new(addr:String) -> Self{

        Self{   //Server
            addr //addr:addr
        }
    }

   fn run(self){    // similiar to this in c++
        println!("Listenting on {}",self.addr);
    }
}