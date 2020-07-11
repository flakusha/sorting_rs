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
/// Bottom-up version is modified version of this algorithm with decreased
/// number of comparisons require function call or complex logic, then bottom-up
/// version of algorithm is more effective
///
/// # Examples
/// ```rust
/// let mut vec = vec![5, 2, 7, 3, 9];
/// sorting_rs::heap_sort(&mut vec);
/// debug_assert_eq!(vec, &[2, 3, 5, 7, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::heap_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
/// ```rust
/// let mut vec = vec![5, 2, 7, 3, 9];
/// sorting_rs::heap_bottom_up_sort(&mut vec);
/// debug_assert_eq!(vec, &[2, 3, 5, 7, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::heap_bottom_up_sort(&mut strings);
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

pub fn heap_bottom_up_sort<T: PartialOrd>(input: &mut [T]) {
    let in_len = input.len();
    for start in (0..=(in_len - 2) / 2).rev() {
        hbu_sift(input, start, in_len - 1);
    }
    for end in (1..in_len).rev() {
        input.swap(end, 0);
        hbu_sift(input, 0, end - 1);
    }
}

fn hbu_leaf_search<T: PartialOrd>(input: &mut [T], start: usize, end: usize)
-> usize {
    let mut current = start;

    loop {
        let child = current * 2 + 1; // Left leaf

        if (child + 1) > end {
            break;
        }
        if input[child + 1] > input[child] {
            current = child + 1;
        }
        else {
            current = child;
        }
    }
    let child = current * 2 + 1; // Only left leaf is present
    if child <= end {
        current = child;
    }
    current
}

fn hbu_sift<T: PartialOrd>(input: &mut [T], start: usize, end: usize) {
    let mut current = hbu_leaf_search(input, start, end);
    
    while input[start] > input[current] {
        current = (current - 1) / 2;
    }
    input.swap(current, start);

    while current > start {
        current = (current - 1) / 2;
        input.swap(current, start);
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
    #[test]
    fn test_heap_bottom_up() {
        let mut array = [10, 20, 11, 24, 22, 21, 19];
        heap_bottom_up_sort(&mut array);
        debug_assert_eq!(array, [10, 11, 19, 20, 21, 22, 24]);
    }
}