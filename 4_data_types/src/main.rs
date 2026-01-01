use std::io;

fn main() {
    let x: i32 = -25;
    let y: u32 = 25;

    println!("x is {x}");
    println!("y is {y}");

    let hex_num = 0xff;
    let oct_num = 0o77;
    let bin_num = 0b1111_0000;

    println!("hex_num is {hex_num}");
    println!("oct_num is {oct_num}");
    println!("bin_num is {bin_num}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum is {sum}");
    println!("difference is {difference}");
    println!("product is {product}");
    println!("quotient is {quotient}");
    println!("truncated is {truncated}");
    println!("remainder is {remainder}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c is {c}");
    println!("z is {z}");
    println!("heart_eyed_cat is {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    let a = [3; 5];

    println!("The value of a is: {:?}", a);

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
