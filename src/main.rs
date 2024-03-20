use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let ipv4 = IP::V4(0x7f000001);
    let ipv6 = IP::V6("::1".to_string());

    ipv4.print();
    ipv6.print();

    option();
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

fn slice_type() {
    let s = String::from("hello world!");
    let first = first_word(&s);

    println!("{first}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn rectangle() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    dbg!(&rect1);

    let square1 = Rectangle::square(5);
    dbg!(&square1);

    println!("rect1 size: {}", rect1.area());
    println!("square1 size: {}", square1.area());
    println!("Can rect1 hold square1: {}", rect1.can_hold(&square1));
}

#[derive(Debug)]
enum IP {
    V4(u32),
    V6(String),
}

impl IP {
    fn print(&self) {
        match self {
            V4 => {
                println!("{:?}", self);
            }
            V6 => {
                println!("{:?}", self);
            }
        }
    }
}

fn option() {
    let some_int = Some(3);
    let some_string = Some(String::from("value"));
    let none_string: Option<String> = None;

    let some_ipv4 = IP::V4(0);
    let some_ipv6 = IP::V6("0".to_string());

    let plus_one: Option<i32> = {
        match some_int {
            None => None,
            Some(i) => Some(i + 1),
        }
    };

    if let Some(i) = plus_one {
        println!("{}", i);
    }
}
