use std::io;
use std::io::Write;

fn main() {
   println!("Power Calculator");
   println!("--------------------");

    loop {
        // Display menu
        println!("\nAvailable Operations:");
        println!("1. Exponential (base^exponent)");
        println!("2. Addition (a + b)");
        println!("3. Subtraction (a - b)");
        println!("4. Multiplication (a * b)");
        println!("5. Division (a / b)");
        println!("6. Square Root (√a)");
        println!("7. Exit");
        let choice = get_operation_choice();

        match choice {
            1 => perform_exponential(),
            7 => {
                println!("THank you for your operation!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn choose_mathematical_operation(prompt: &str) {
    match prompt {
        "exponential" => {
            let base = get_number_from_user("Enter the base number: ");

            let exponent = get_number_from_user("Enter the exponent: ");
        },
        _ => println!("Please enter a math operation"),
    }
}

fn get_operation_choice() -> usize {
    loop {
        print!("\nEnter your choice (1-7): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => {
                        println!("Please enter a valid number (1-7)");
                        continue;
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                continue;
            }
        }
    }
}

fn perform_exponential() {
    println!("\n--- Exponential Calculation (base^exponent) ---");
    let base = get_number_from_user("Enter the base number: ");
    let exponent = get_number_from_user("Enter the exponent: ");

    // Calculate the power
    let result = base.powf(exponent);

    // Display the result
    println!("\nResult:");
    if (base * 10.0).fract() == 0.0 && exponent.fract() == 0.0 && result.fract() == 0.0 {
        println!("{}^{} = {:.0}", base, exponent, result);
    } else {
        println!("{}^{} = {:.6}", base, exponent, result);
    }
}

fn get_number_from_user(prompt: &str) -> f64 {
    loop {
        print!("\n{} ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                match user_input.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => {
                        println!("Please enter a number (eg 4.5, 3, -2.7)");
                        continue;
                    }
                }
            },
            Err(error) => {
                println!("error: {}", error);
                continue;
            }
        }
    }
}