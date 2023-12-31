
use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

   // println!("The secret number is: {secret_number}");

   loop {
    println!("Please input your guess.");
    
   
    

   // let mut guess = String::new();

   // io::stdin()
    //    .read_line(&mut guess)
     //   .expect("Failed to read line");
   
   
   
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Kindly enter a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Greater =>println!("Too Big"),
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => {
            println!("You have WON!");
            break;

        }
    }
        
    }
}