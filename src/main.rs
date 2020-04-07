
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Input your number:");

    let secret_number = rand::thread_rng().gen_range(1,101);



    let mut number = String::new();



    io::stdin().read_line(&mut number)
        .expect("Error");

    let number: u32 = number.trim().parse()
        .expect("Input number");

    println!("Your name is {}", number);
    println!("Secret number is {}", secret_number);

    match number.cmp(&secret_number){
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Big"),
        Ordering::Equal => println!("You Win")
    }

}
