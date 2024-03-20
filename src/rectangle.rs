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

pub fn rectangle() {
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
