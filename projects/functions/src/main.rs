fn main() {
    println!("Hello, world!");

    another_function(1);
    print_labeled_measurement(1,'🐒');
    
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    let x = plus_one(5);
    println!("The value of x is: {x}");
    while_loop();
    for_loop();
    rev_for_loop();
    
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn rev_for_loop() {
    (1..4).rev().into_iter().for_each(|a| {
        println!("{a}!");
    });
    println!("LIFTOFF!!!");
}