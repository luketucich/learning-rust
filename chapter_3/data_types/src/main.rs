use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.").

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

    // // Data types
    // // We  must add u32 because multiple types are possible
    // let guess: u32 = "42".parse().expect("Not a number!");

    // println!("You guessed {guess}");

    // // Scalar types
    // // integers, floating-point numbers, Booleans, characters

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // let t = true;

    // let f: bool = false; // with explicit type annotation

    // // Rust’s char type is four bytes in size and represents
    // // a Unicode Scalar Value, which means it can represent a
    // // lot more than just ASCII. Accented letters; Chinese,
    // // Japanese, and Korean characters; emoji; and zero-width
    // // spaces are all valid char values in Rust.

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // // Compound types
    // // A tuple is a general way of grouping together a number
    // // of values with a variety of types into one compound type.
    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {y}");

    // // We can also access a tuple element directly by using a period (.)
    // // followed by the index of the value we want to access. For example:
    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // // Arrays
    // let a = [1,2,3,4,5]

    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // type is i32, num elements is 5

    // let a = [3; 5]; // creates an array filled with 5 elements that are 3
}
