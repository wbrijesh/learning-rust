use std::io;

fn main() {
    println!("Simple Rust CLI Calculator");

    loop {
        let choice = get_option(&[
            "Add",
            "Subtract",
            "Multiply",
            "Divide",
            "Exit",
        ]);

        match choice {
            0 => add(),
            1 => subtract(),
            2 => multiply(),
            3 => divide(),
            _ => break,
        }
    }
}

fn add() {
    println!("Addition");

    let first_number: i32 = get_input("Enter the first number:");
    let second_number: i32 = get_input("Enter the second number:");

    let result = first_number + second_number;

    println!("The result is {}", result);
}

fn subtract() {
    println!("Subtraction");

    let first_number: i32 = get_input("Enter the first number:");
    let second_number: i32 = get_input("Enter the second number:");

    let result = first_number - second_number;

    println!("The result is {}", result);
}

fn multiply() {
    println!("Multiplication");

    let first_number: i32 = get_input("Enter the first number:");
    let second_number:i32 = get_input("Enter the second number:");

    let result = first_number * second_number;

    println!("The result is {}", result);
}

fn divide() {
    println!("Division");

    let first_number: i32 = get_input("Enter the first number:");
    let second_number:i32 = get_input("Enter the second number:");

    let result = first_number / second_number;

    println!("The result is {}", result);
}

fn get_input<T>(message: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn get_option(options: &[&str]) -> i16 {
    println!("Choose an option:");

    for (index, option) in options.iter().enumerate() {
        println!("{}. {}", index + 1, option);
    }

    let choice: i16 = get_input("Enter the number corresponding to the option:");

    if choice >= 1 && choice <= options.len().try_into().unwrap() {
        return choice - 1;
    } else {
        return -1;
    }
}


