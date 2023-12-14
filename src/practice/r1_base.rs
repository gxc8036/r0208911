use std::{cmp::Ordering, io};

use rand::Rng;

#[test]
fn guess_num_game() {
    println!("guess the mum game.");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("please enter the number (entering quit will terminate the program ): ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        let guess = guess.trim();

        if guess == "quit" {
            println!("the program terminated.");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("parse err: {}", err);
                continue;
            }
        };

        println!("you guess num is : {guess} ");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
