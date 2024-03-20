pub fn slice_type() {
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
