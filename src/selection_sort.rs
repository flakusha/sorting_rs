/// Sorts a slice in-place using
/// [Selection sort](https://en.wikipedia.org/wiki/Selection_sort).
/// [Double selection sort](http://warp.povusers.org/DoubleBurstSelectionSort/).
/// All kinds of slices can be sorted as long as they implement
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
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::selection_double_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::selection_double_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn selection_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

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

pub fn selection_double_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let mut left = 0;
    let mut right = input.len() - 1;
    let mut min = left;
    let mut max = left;

    while left <= right {
        for i in left..=right {
            if input[i] > input[max] {
                max = i;
            }
            if input[i] < input[min] {
                min = i;
            }
        }
        if max == left {max = min;}
        input.swap(left, min);
        input.swap(right, max);

        left += 1;
        right -= 1;

        min = left;
        max = right;
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
    #[test]
    fn test_selection_empty() {
        let mut vector_in:Vec<i32> = vec![];
        selection_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_selection_len1() {
        let mut vector_in = vec![1];
        selection_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_selection_double() {
        let mut vector_in = vec![11, 20, 21, 40, 11, 60, 5];
        selection_double_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![5, 11, 11, 20, 21, 40, 60]);
    }
    #[test]
    fn test_selection_double_empty() {
        let mut vector_in:Vec<i32> = vec![];
        selection_double_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_selection_double_len1() {
        let mut vector_in = vec![1];
        selection_double_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}