use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess: ");

        // instantiate guess as a string
        let mut guess = String::new();

        // read user input into guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // must trim() the newline from the guess string
        // must parse() the string into a num
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catchall for error values
            // continue goes to the next iteration of the loop and asks for a new guess
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
