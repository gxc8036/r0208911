use std::{cmp::Ordering, io};

use rand::Rng;

#[test]
fn r1_test() {
    println!("hello world");
}

#[test]
fn guess_num_game() {
    println!("guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        print!("please enter a secret number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("failed to parse error: {}", err);
                continue;
            }
        };

        println!("you guessed: {guess} ");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
