//! Additional private binary to print powers of two.
//! These numbers are used in bitonic sort algorithm as constant.
//! This one can be useful in case you need to modify algorithm to use with
//! 32-, 64-, 128-bit and other systems.
//! This addition uses usize in case there is mainstream 64-bit system.
//! 
//! # Usage:
//! ```text
//! cargo run --bin powers_of_two
//! ```
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read value");
    input = input.to_string().trim().to_string();
    match input.parse::<usize>() {
        Ok(input) => {calculate_powers_of_two(input)},
        _ => {println!("Input is not a number!"); main();}
    }
}

fn calculate_powers_of_two(input: usize) {
    let mut powers = Vec::<usize>::with_capacity(input);
    for i in 1..input + 1 {
        powers.push(2usize.pow(i as u32));
    }
    println!("{:?}", powers);
}