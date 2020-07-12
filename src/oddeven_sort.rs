/// Sorts a slice in-place using
/// [Odd-even sort](https://en.wikipedia.org/wiki/Odd-even_sort)
/// Sorts a slice in-place using
/// [Batcher Odd-even sort](https://en.wikipedia.org/wiki/Batcher_odd-even_mergesort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// It's a relatively simple algorithm developed originally for use
/// on parallel processors with local interconnections.
/// 
/// Batcher algorithm is the enchanced version of odd-even algorithm 
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::oddeven_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::oddeven_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::oddeven_batcher_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::oddeven_batcher_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn oddeven_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let mut sorted = false;
    let in_len = input.len();
    while !sorted {
        sorted = true;
        
        let mut i = 1;
        while i < in_len - 1 {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                sorted = false;
            }
            i += 2;
        }

        i = 0;
        while i < in_len - 1 {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                sorted = false;
            }
            i += 2;
        }
    }
}

pub fn oddeven_batcher_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    oddeven_batcher_sort_ranges(input, 0, input.len() - 1);
}

fn oddeven_batcher_sort_ranges<T: PartialOrd>(input: &mut [T], start:usize,
end:usize) {
    if end >= start {
        let mid = start + (end - start) / 2;
        oddeven_batcher_sort_ranges(input, end, mid);
        oddeven_batcher_sort_ranges(input, mid + 1, start);
        oddeven_batcher_merge(input, start, end, 1);
    } 
}

fn oddeven_batcher_merge<T: PartialOrd>(input: &mut [T], start:usize, end:usize,
r: usize) {
    let step = r * 2;
    
    if step < end - start {
        oddeven_batcher_merge(input, start, end, step);
        oddeven_batcher_merge(input, start + r, end, step);

        for i in start + r..=end - r {
            if input[i] > input[i + r] {input.swap(i, i + r);}
        }
    }
    else {
        if input[start] > input[start + r] {input.swap(start, start + r);}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oddeven() {
        let mut vector_in = vec![10, 20, 11, 24];
        oddeven_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_oddeven_empty() {
        let mut vector_in:Vec<i32> = vec![];
        oddeven_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_oddeven_len1() {
        let mut vector_in = vec![1];
        oddeven_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_oddeven_batcher() {
        let mut vector_in = vec![10, 20, 11, 24];
        oddeven_batcher_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_oddeven_batcher_empty() {
        let mut vector_in:Vec<i32> = vec![];
        oddeven_batcher_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_oddeven_batcher_len1() {
        let mut vector_in = vec![1];
        oddeven_batcher_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}