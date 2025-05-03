fn main() {
    print_labeled_measurement(5, 'h');

    // This will return an error because statements don't return anything
    // let x = (let y = 6);

    let y = {
        let x = 3; // statement (nothing returned)
        x + 1 // expression, no semicolon, returns 4
    };

    println!("The value of y is: {y}");

    // Should return 5
    let x = five();
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with return value
fn five() -> i32 {
    // must specify return type with arrow
    5
}
