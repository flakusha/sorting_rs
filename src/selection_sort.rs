/// Sorts a slice in-place using
/// [Selection sort](https://en.wikipedia.org/wiki/Selection_sort).
/// /// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// # Examples
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::selection_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::selection_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn selection_sort<T: PartialOrd>(input: &mut [T]) {
    for i in 0..input.len() {
        let swap_val = {
            let mut min = &input[i];
            let mut index_min = i;
            
            for j in i + 1..input.len() {
                if input[j] < *min {
                    min = &input[j];
                    index_min = j;
                }
            }
            index_min
        };

        if i != swap_val {
            input.swap(i, swap_val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection() {
        let mut vector_in = vec![11, 20, 21, 40, 11, 60, 5];
        selection_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![5, 11, 11, 20, 21, 40, 60]);
    }
}