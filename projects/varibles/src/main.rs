use std::io;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    shadow();
    data_type();
    tuple_example();
    array_example1();
    array_example2();
}
// shadowing
fn shadow() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len(); cannot change the variable type
}

fn data_type() {
    let guess: u32 = "42".parse().expect("Not a number!");
}

fn tuple_example() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // destucturing

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn array_example1() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
}


fn array_example2() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}