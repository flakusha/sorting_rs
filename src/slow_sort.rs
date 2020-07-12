/// Sorts a slice in-place using
/// [Slow sort](https://en.wikipedia.org/wiki/Slowsort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// It's of humorous nature and not useful.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::slow_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::slow_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn slow_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    slow_sort_sorting(input, 0, input.len() - 1);
}

fn slow_sort_sorting<T: PartialOrd>(input: &mut [T], start: usize, end: usize) {
    if start >= end {return;}

    let mid = (start + end) / 2;
    slow_sort_sorting(input, start, mid);
    slow_sort_sorting(input, mid + 1, end);
    if input[end] < input[mid] {input.swap(end, mid);}
    slow_sort_sorting(input, start, end - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow() {
        let mut vector_in = vec![10, 20, 11, 24];
        slow_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_slow_empty() {
        let mut vector_in:Vec<i32> = vec![];
        slow_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_slow_len1() {
        let mut vector_in = vec![1];
        slow_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}