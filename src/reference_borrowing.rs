pub fn reference_borrowing() {
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
