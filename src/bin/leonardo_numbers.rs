/// Additional private binary to print Leonardo and Fibonnaci numbers
/// Leonardo numbers are then used in smoothsort algorithm
/// This one can be useful in case you need to modify algorithm to use with
/// 32-, 64-, 128-bit and other systems
/// This addition uses usize in case there is mainstream 64-bit system
/// # Usage:
/// ```no_run
/// cargo run leonardo_numbers
/// ```
use std::io;

fn leonardo_generate(mut n0: usize, mut n1: usize, add: usize) ->
impl std::iter::Iterator<Item = usize> {
    std::iter::from_fn(move || {
        let n = n0;
        n0 = n1;
        n1 += n + add;
        Some(n)
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read value");
    input = input.to_string().trim().to_string();
    match input.parse::<usize>() {
        Ok(input) => {calculate_leonardo(input)},
        _ => {println!("Input is not a number!"); main();}
    }
}

fn calculate_leonardo(num: usize) {
    for i in leonardo_generate(1, 1, 1).take(num) {
        print!("{}, ", i);
    }
    println!();
    println!("Maximum number: {}", usize::MAX);
}

/*
fn calculate_fibonacci(num: usize) {
    for i in leonardo_generic(0, 1, 0).take(num)
}
*/