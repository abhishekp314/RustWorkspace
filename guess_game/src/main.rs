extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let random_number = rand::thread_rng().gen_range(1, 101);
    //println!("Secret la number est {}", random_number);

    println!("Guess la number");

    loop {
        println!("Silvupla input une guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("[IO] Error reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("Toi guessed : {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Toi win");
                break;
            }
        }
    }

}
