
use std::io;

fn main() {

    println!("Input your name:");

    let mut name = String::new();

    io::stdin().read_line(&mut name)
        .expect("Error");

    println!("Your name is {}", name);


}
