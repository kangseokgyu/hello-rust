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

pub fn option() {
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
