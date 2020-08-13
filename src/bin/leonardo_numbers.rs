//! Additional private binary to print Leonardo and Fibonnaci numbers.
//! Leonardo numbers are used in smoothsort algorithm as constant.
//! This one can be useful in case you need to modify algorithm to use with
//! 32-, 64-, 128-bit and other systems.
//! This addition uses usize in case there is mainstream 64-bit system.
//! 
//! # Usage:
//! ```text
//! cargo run --bin leonardo_numbers
//! ```
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read value");
    input = input.to_string().trim().to_string();
    println!("Maximum number: {}\n", usize::MAX);
    match input.parse::<usize>() {
        Ok(input) => {calculate_leonardo(input); calculate_fibonacci(input);},
        _ => {println!("Input is not a number!"); main();}
    }
}

fn leonardo_generate(mut n0: usize, mut n1: usize, add: usize) ->
impl std::iter::Iterator<Item = usize> {
    std::iter::from_fn(move || {
        let n = n0;
        n0 = n1;
        n1 += n + add;
        Some(n)
    })
}


fn calculate_leonardo(num: usize) {
    println!("Leonardo numbers:");
    for i in leonardo_generate(1, 1, 1).take(num) {
        print!("{}, ", i);
    }
    println!();
}

fn calculate_fibonacci(num: usize) {
    println!("Fibonacci numbers:");
    for i in leonardo_generate(0, 1, 0).take(num) {
        print!("{}, ", i);
    }
    println!();
}