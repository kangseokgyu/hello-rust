pub fn control_flow() {
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
