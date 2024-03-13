use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    control_flow();
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let mut counter = 0;
    let number = 'get_counter: loop {
        if counter == 10 {
            break 'get_counter counter;
        }
        counter += 1;
    };
    println!("number is {number}.");
}
