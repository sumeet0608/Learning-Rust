use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess a Number!");

    let secret_number = rand::thread_rng().gen_range(0..100);
    //println!("Secret Number is: {}", secret_number);    
    loop{

        println!("Input your guess: ");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");
    
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Equal => {
                println!("{}","You guessed it!!".green());
                break;
            },
        };
    }           
}
