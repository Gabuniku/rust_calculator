/**
 * Rust Calculator
 * main.rs
 * (c) 2022 Gabuniku
 */
use std::{
    io::{BufRead, Write},
    num::ParseIntError,
};

mod token;

fn menu() -> Result<i32, ParseIntError> {
    println!("Select Calc Mode.");
    println!("(0) | Exit");
    println!("(1) | Calc");
    print!("Mode >>");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let r = input.parse()?;
    Ok(r)
}

fn calc() {
    print!(">>");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let result = token::execute(input).unwrap();
    println!("{}", result);
}

fn main() {
    println!("+--------------------------------+");
    println!("| calculator by Gabuniku on Rust |");
    println!("+--------------------------------+");
    loop {
        let mode = match menu() {
            Err(_err) => {
                println!("Error");
                continue;
            }
            Ok(i) => i,
        };

        match mode {
            0 => break,
            1 => calc(),
            _ => (),
        }
    }
    println!("Bye.")
}
