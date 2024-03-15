use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    reference_borrowing();
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

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }
    println!("counter is {counter}.");

    for i in 1..10 {
        println!("{i}");
    }

    let numbers = [10, 20, 30, 40, 50];
    for number in numbers {
        println!("{number}");
    }
}

fn reference_borrowing() {
    let mut s1 = String::from("value");

    let r1 = &s1;
    let r2 = &s1;

    println!("{} {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);

    /* dangling reference
    let s2 = {
        let s = String::from("dangle");
        &s
    };
    */
}
