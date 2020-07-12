/// Sorts a slice in-place using
/// [Stooge sort](https://en.wikipedia.org/wiki/Stooge_sort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Well, it's a bit faster than slow sort...
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::stooge_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::stooge_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn stooge_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    stooge_sort_sorting(input, 0, input.len() - 1);
}

fn stooge_sort_sorting<T: PartialOrd>(input: &mut [T], start: usize,
end: usize) {
    if input[start] > input[end] {input.swap(start, end);}
    if start <= end {
        if (end - start + 1) > 2 {
            let temp = (end - start + 1) / 3;
            stooge_sort_sorting(input, start, end - temp);
            stooge_sort_sorting(input, start + temp, end);
            stooge_sort_sorting(input, start, end - temp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stooge() {
        let mut vector_in = vec![10, 20, 11, 24];
        stooge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_slow_empty() {
        let mut vector_in:Vec<i32> = vec![];
        stooge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_slow_len1() {
        let mut vector_in = vec![1];
        stooge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}