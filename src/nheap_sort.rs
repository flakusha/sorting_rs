/// Sorts a slice in-place using
/// [N-heap sort](https://en.wikipedia.org/wiki/Heapsort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// 3 children are a bit more effective than 2, though 4 and more are generally
/// less effective, but you can modify this parameter by editing souces.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::nheap_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::nheap_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn nheap_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let children = 3;
    for i in (0..=input.len()).rev() {
        nheap_sift(input, children, i, input.len() - 1);
    }

    for i in (1..input.len()).rev() {
        input.swap(i, 0);
        nheap_sift(input, children, 0, i - 1);
    }
}

fn nheap_sift<T: PartialOrd>(input: &mut [T], children: usize, start: usize,
end: usize) {
    let mut root = start;
    
    loop {
        let child = root * children + 1;
        if child > end {break;}
        let mut max = child;

        for k in 2..children + 1 {
            let current = root * children + k;
            if current > end {break;}

            if input[current] > input[max] {
                max = current;
            }
        }
        if input[root] < input[max] {
            input.swap(root, max);
            root = max;
        } else {break;}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut vector_in = vec![10, 20, 11, 24];
        nheap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_heap_01() {
        let mut vector_in = vec![10, 20, 11, 24, 13, 12];
        nheap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 12, 13, 20, 24]);
    }
    #[test]
    fn test_heap_empty() {
        let mut vector_in:Vec<i32> = vec![];
        nheap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_heap_len1() {
        let mut vector_in = vec![1];
        nheap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}