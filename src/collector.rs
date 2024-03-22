use std::collections::HashMap;

pub fn collector() {
    vector();
    string();
    hashmap();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    String(String),
}

fn vector() {
    println!("[Vector]");
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    println!("{:?}", v);

    let third = v[2];
    let third = v.get(2);
    match third {
        None => println!("Third number is None"),
        Some(i) => println!("Third nubmer is {}", i),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("{i}");
    }

    let spreadsheet = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::String("value".to_string()),
    ];

    for c in &spreadsheet {
        println!("{:?}", c);
    }
}

fn string() {
    println!();
    println!("[String]");
    let mut s1 = String::from("안녕하세요~!");
    let s2 = "반갑습니다.".to_string();

    println!("{}", s1);
    println!("{}", s2);

    s1.push(' ');
    s1.push_str(&s2);
    println!("{}", s1);

    s1 += &s2;
    println!("{}", s1);

    s1 = format!("{s1} {s2}");
    println!("{}", s1);

    for c in s1.chars() {
        print!("{}", c);
    }
    println!();
    for b in s1.bytes() {
        print!("{:#x} ", b);
    }
    println!();
}

fn hashmap() {
    println!();
    println!("[HashMap]");
    let mut h = HashMap::new();

    h.insert("BLUE".to_string(), 10);
    h.insert("RED".to_string(), 20);

    let blue_score = h.get(&String::from("BLUE")).copied().unwrap_or(0);

    for (k, v) in &h {
        println!("team: {k}, score: {v}");
    }
    println!();

    h.entry("BLUE".to_string()).or_insert(100);
    h.entry("YELLOW".to_string()).or_insert(100);

    for (k, v) in &h {
        println!("team: {k}, score: {v}");
    }
    println!();

    let s = "hello world wonderful world";
    let mut h1 = HashMap::new();

    for c in s.split_whitespace() {
        let count = h1.entry(c).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", h1);

    let mut h2 = HashMap::new();

    for c in s.chars() {
        let count = h2.entry(c).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", h2);
}
