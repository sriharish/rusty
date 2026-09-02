use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // prompt
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess.");

    loop {
        // process guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow variable of guess input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // output guess
        println!("You guessed: {}", guess);

        // evaluate
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
