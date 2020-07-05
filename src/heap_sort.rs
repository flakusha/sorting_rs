/// Sorts a slice in-place using
/// [Heap sort](https://en.wikipedia.org/wiki/Heapsort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Heap sort is basically an improved version of selection sort. Where the
/// selection is now done in logarithmic time instead of linear.
///
/// First it transforms the array into a max-heap and then swaps the first
/// element with the last element of the array, effectively shrinking the
/// heap. Then it must max heapify again since the swapped value is smaller
/// than the original max value. This process is repeated until there is no
/// heap left.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5, 2, 7, 3, 9];
/// sorting_rs::heap_sort(&mut vec);
/// debug_assert_eq!(vec, &[2, 3, 5, 7, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::shell_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn heap_sort<T: PartialOrd>(input: &mut [T]) {

    for i in (0..input.len() / 2).rev() {
        heap_max(input, i, input.len());
    }

    for i in (0..input.len()).rev() {
        input.swap(0, i);
        heap_max(input, 0, i);
    }
}

/// Max heapifies an embedded heap from given index.
fn heap_max<T: PartialOrd>(input: &mut [T], i: usize, heap_len: usize) {
    let left = 2 * i + 1;
    let right = left + 1;

    let mut largest = i;
    if left < heap_len && input[left] > input[largest] {
        largest = left;
    }
    if right < heap_len && input[right] > input[largest] {
        largest = right;
    }

    if largest != i {
        input.swap(i, largest);
        heap_max(input, largest, heap_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut vector_in = vec![10, 20, 11, 24];
        heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
}