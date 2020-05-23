use std::io;
use std::io::prelude::{Write, Read};
use std::cmp::Ordering;
use rand::Rng;

// To pause at the end of program. Useful if running a Windows
// exe. Without pause the command line would exit without
// displaying the result
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line,
    // so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

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
                pause();
                break;
            }
        }
    }
}