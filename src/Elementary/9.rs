extern crate rand;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn main() {
    let mut guess: Vec<u32> = Vec::new();

    let mut rng = thread_rng();
    let num: u32 = rng.gen_range(1, 100);
    print!("Please enter a number from [1, 100): ");
    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }

        let guess_num: u32;
        match input.trim().parse() {
            Ok(n) => guess_num = n,
            Err(e) => panic!("Error: {}", e),
        }
        if guess.is_empty() || guess_num != guess[guess.len() - 1] {
            guess.push(num)
        }

        if guess_num < num {
            println!("Too small!")
        } else if guess_num > num {
            println!("Too big")
        } else {
            println!("Congratulations!, {} times", guess.len());
            break;
        }
    }
}
