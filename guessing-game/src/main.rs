extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess my fancy-ass number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Number plz brotato");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Shit's broken yo");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that is not a number sir, and you are no gentleman!");
                continue;
            }
        };

        println!("here's that number you gave me broBama: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You am the winrar!");
                break;
            }
        }
    }
}
