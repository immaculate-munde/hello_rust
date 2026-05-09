use std::io::{self, Write};

fn main() {
    let mut balance: f64 = 0.0;

    println!("--- Welcome to Simple Balance Tracker ---");
    println!("Commands: add <amount>, sub <amount>, exit");

    loop {
        print!("\nCurrent Balance: ${:.2}\n> ", balance);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Usage: add <amount> or sub <amount>");
            continue;
        }

        let command = parts[0];
        let amount_str = parts[1];

        // Use our refactored function to process the logic
        match process_transaction(balance, command, amount_str) {
            Ok(new_balance) => {
                let diff = (new_balance - balance).abs();
                if command == "add" {
                    println!("Added ${:.2}", diff);
                } else {
                    println!("Subtracted ${:.2}", diff);
                }
                balance = new_balance;
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Final balance: ${:.2}. Goodbye!", balance);
}

/// Processes a single transaction logic.
/// Returns the NEW balance on success, or an error string on failure.
fn process_transaction(current_balance: f64, command: &str, amount_str: &str) -> Result<f64, String> {
    let amount = amount_str
        .parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number.", amount_str))?;

    if amount <= 0.0 {
        return Err("Amount must be a positive number.".to_string());
    }

    match command {
        "add" => Ok(current_balance + amount),
        "sub" => {
            if amount <= current_balance {
                Ok(current_balance - amount)
            } else {
                Err("Insufficient balance! (Operation blocked)".to_string())
            }
        }
        _ => Err(format!("Unknown command '{}'.", command)),
    }
}

// Tests to ensure our logic works and prevents invalid operations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_balance() {
        let res = process_transaction(10.0, "add", "5.5");
        assert!(res.is_ok());
        assert!((res.unwrap() - 15.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_subtract_balance() {
        let res = process_transaction(10.0, "sub", "3.0");
        assert!(res.is_ok());
        assert!((res.unwrap() - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_insufficient_funds() {
        let res = process_transaction(10.0, "sub", "15.0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Insufficient balance! (Operation blocked)");
    }

    #[test]
    fn test_invalid_number() {
        let res = process_transaction(10.0, "add", "abc");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("not a valid number"));
    }

    #[test]
    fn test_negative_amount() {
        let res = process_transaction(10.0, "add", "-5.0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Amount must be a positive number.");
    }
}
