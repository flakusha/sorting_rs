/// Sorts a slice in-place using
/// [Cycle sort](https://en.wikipedia.org/wiki/Cycle_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// This kind of sort is pretty good in case you need as low writes as
/// possible, for example, using SSD or NAND memory
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::cycle_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::cycle_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
use std::fmt::Debug;

pub fn cycle_sort<T: PartialOrd + Copy + Debug>(input: &mut [T]) {
    let in_len = input.len();
    if in_len < 2 {return;}
    
    for item in 0..in_len {
        let key = input[item];
        let mut pos = item;

        for i in item + 1..in_len {
            if input[i] < key {
                pos += 1;
            }
        }
        
        if pos == item {continue;}

        while key == input[pos] {
            pos += 1;
        }
        input.swap(pos, item);
        println!("DBG: 0 {:?}", input);

        while pos != item {
            pos = item;
            for i in item + 1..in_len {
                if input[i] < key {
                    pos += 1;
                }
            }
            println!("DBG: 1 {:?}", input);
            while key == input[pos] {
                pos += 1;
            }
            input.swap(pos, item);
            println!("DBG: 2 {:?}", input);
        }
        println!("DBG: 3 {:?}", input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut vector_in = vec![11, 20, 21, 40, 11, 60, 5];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![5, 11, 11, 20, 21, 40, 60]);
    }
    #[test]
    fn test_cycle_empty() {
        let mut vector_in:Vec<i32> = vec![];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_cycle_len1() {
        let mut vector_in = vec![1];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}