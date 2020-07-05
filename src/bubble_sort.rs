/// Sorts an input slice in-place using
/// [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// # Examples
/// ```rust
/// let mut slice = vec![3,2,1,4];
/// sorting_rs::bubble_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::bubble_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn bubble_sort<T: PartialOrd>(input: &mut [T]) {
    let input_len = input.len();

    for i in (0..input_len).rev() {
        let mut has_swapped = false;
        for j in 0..i {
            if input[j] > input[j + 1] {
                input.swap(j, j + 1);
                has_swapped = true;
            }
        }
        if !has_swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut vector_in = vec![30, 10, 20, 11, 24, 44, 12, 11];
        bubble_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 11, 12, 20, 24, 30, 44]);
    }
}