use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn error_handling() {
    panic();
    read_username_from_file();
}

fn panic() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Failed to create a file. {:?}", error),
            },
            other_error => panic!("Failed to open file. {:?}", other_error),
        },
    };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create a file. {:?}", error);
            })
        } else {
            panic!("Failed to open file. {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username: String = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file__() -> Result<String, io::Error> {
    let mut username: String = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file___() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
