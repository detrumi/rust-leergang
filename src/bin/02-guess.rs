use if_chain::if_chain;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        if_chain! {
            if let Ok(_) = io::stdin().read_line(&mut guess);
            if let Ok(guess) = guess.trim().parse::<u32>();
            if guess <= 100;
            then {
                println!("You guessed: {}", guess);
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
        }
    }
}
