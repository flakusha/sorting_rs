/// Sorts a slice in-place using
/// [Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// This sorting algorithm is very efficient when used on small data sets.
/// This is because insertion sort has constant space complexity and works
/// very fast when used on partially sorted data.
///
/// # Examples
/// ```rust
/// let mut vec = vec![-4, -5, 7, 45, 0];
/// sorting_rs::insertion_sort(&mut vec);
/// assert_eq!(vec, &[-5, -4, 0, 7, 45]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::shell_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn insertion_sort<T: PartialOrd>(input: &mut [T]) {
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut vector_in = vec![10, 20, 11, 24];
        insertion_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
}
