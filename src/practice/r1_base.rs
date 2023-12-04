use std::{cmp::Ordering, io};

use rand::Rng;

#[test]
fn guess_num_game() {
    println!("猜数字游戏!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入你猜的数字 (输入quit将直接终止程序): ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("不能正确读出输入信息！");

        let guess = guess.trim();

        if guess == "quit" {
            println!("程序终止!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("解析失败: {}", err);
                continue;
            }
        };

        println!("你当前输入的数字是: {guess} ");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("胜利!");
                break;
            }
        }
    }
}
