use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number (1-100)!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number.");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is under..."),
            Ordering::Greater => println!("Your guess is over..."),
            Ordering::Equal => {
                println!("You got it!!!");
                break;
            },
        }
    }
}
