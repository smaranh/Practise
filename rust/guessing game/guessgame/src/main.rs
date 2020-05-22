use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Lets play a guessing game");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("Input your guess");
        io::stdin()
        .read_line(&mut guess)
        .expect("Could not read your guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Your guess is low"),
            Ordering::Greater => println!("Your guess is high"),
            Ordering::Equal => {
                println!("You WIN!!");
                break;
            }
        }
    }
}