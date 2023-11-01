use rand::Rng; // 随机数
use std::cmp::Ordering; // 比较
use std::io; // 使用 io 标准库

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_num);

    loop {
        println!("Input the number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
