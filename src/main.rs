mod brzozowski_derivative;

use brzozowski_derivative::parse;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("Regex: ");
        stdout().flush().unwrap();
        let mut regex_str = String::new();
        stdin().read_line(&mut regex_str).unwrap();
        regex_str = regex_str.trim_end().to_string();

        print!("Test: ");
        stdout().flush().unwrap();
        let mut test_str = String::new();
        stdin().read_line(&mut test_str).unwrap();
        test_str = test_str.trim_end().to_string();

        match parse(&regex_str) {
            Ok(mut regex) => {
                println!("Regex: {:?}", regex);
                for ch in test_str.chars().collect::<Vec<char>>() {
                    regex = regex.derivative(ch);
                    println!("Derivative by '{}': {:?}", ch, regex);
                }

                if regex.is_accept_empty_str() {
                    println!("Mached!");
                } else {
                    println!("Don't mached!")
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
