/// Sorts a slice in-place using
/// [Pancake sort](https://en.wikipedia.org/wiki/Pancake_sorting).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// This algorithm aims to minimize number of comparisons, though amount of
/// data swaps is pretty high, which doesn't make it very effective in practical
/// use
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::pancake_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::pancake_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn pancake_sort<T: PartialOrd + Copy>(input: &mut Vec<T>) {
    if input.len() < 2 {return;}

    let in_len = input.len() - 1;
    for item in (0..in_len + 1).rev() {
        let cut = largest_pancake(input, item);

        flip(input, cut);
        flip(input, item);
    }
}

fn largest_pancake<T: PartialOrd + Copy>(input: &[T], index: usize)
-> usize {
    let mut pancake = input[index];
    let mut largest = index;

    for i in 0..index {
        if input[i] > pancake {
            pancake = input[i];
            largest = i;
        }
    }
    largest
}

fn flip<T: PartialOrd + Copy>(input: &mut Vec<T>, index: usize) {
    let mut pile:Vec<T> = input.drain(0..=index).rev().collect();
    pile.append(input);
    std::mem::swap(input, &mut pile);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pancake() {
        let mut vector_in = vec![10, 20, 11, 24, 15];
        pancake_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 15, 20, 24]);
    }
    #[test]
    fn test_pancake_empty() {
        let mut vector_in:Vec<i32> = vec![];
        pancake_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_pancake_len1() {
        let mut vector_in = vec![1];
        pancake_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}