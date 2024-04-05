

use std::{io::stdin};

pub fn cls() -> (){
    print!("{}[2J", 27 as char);
}

pub fn print_vec(vec : Vec<String>) -> () {
    for v in vec{
        println!("{}", v.to_string());
    }
}


pub fn get_input() -> String{
    let mut user_input = String::new();
    stdin().read_line(&mut user_input);
    return user_input;
}