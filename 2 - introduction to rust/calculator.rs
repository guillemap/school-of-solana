use std::io;

fn main() {
    // Prompt the user to enter two numbers
    println!("Enter two numbers in two separate lines:");

    let num1 = read_float_input();
    let num2 = read_float_input();

    // Prompt the user to choose an operation
    println!("Choose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let operation = read_int_input();

    // Calculate the result based on the chosen operation
    let result = match operation {
        1 => add(num1, num2),
        2 => subtract(num1, num2),
        3 => multiply(num1, num2),
        4 => divide(num1, num2),
        _ => {
            println!("Invalid operation");
            std::process::exit(1);
        }
    };

    // Print the result
    println!("Result: {}", result);
}

// Function to add two numbers
fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

// Function to subtract two numbers
fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

// Function to multiply two numbers
fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

// Function to divide two numbers
fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        println!("Error: Division by zero");
        std::process::exit(1);
    }
    num1 / num2
}

// Function to read a float input from the user
fn read_float_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        }
    }
}

// Function to read an integer input from the user
fn read_int_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        }
    }
}
