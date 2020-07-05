/// Sorts a slice in-place using
/// [Comb sort](https://en.wikipedia.org/wiki/Comb_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// Comb sort is a simple algorithms that improves on bubble sort. It does so
/// by eliminating small values at the end of the list quickly, since these
/// slow down bubble sort. This is solved by comparing elements at the
/// beginning of the list to the end with a so-called gap. As the algorithm
/// the gap size shrinks until it is finally 1, where it is the same as bubble
/// sort.
///
/// # Examples
/// ```rust
/// let mut vec = vec![9, 7, 8, 5, 1];
/// sorting_rs::comb_sort(&mut vec);
/// assert_eq!(vec, &[1, 5, 7, 8, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::comb_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn comb_sort<T: PartialOrd>(input: &mut [T]) {
    let len = input.len();
    let inv_shrink: f32 = 1.0 / 1.3;

    let mut gap = len;
    let mut sorted = len < 2;

    while !sorted {
        gap = (gap as f32 * inv_shrink).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..len - gap {
            if input[i] > input[i + gap] {
                input.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        let mut vector_in = vec![30, 10, 20, 11, 24, 44, 12, 11];
        comb_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 11, 12, 20, 24, 30, 44]);
    }
}