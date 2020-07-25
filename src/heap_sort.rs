/// Sorts a slice in-place using
/// [Heap sort](https://en.wikipedia.org/wiki/Heapsort),
/// [Bottom-up heap sort](https://en.wikipedia.org/wiki/Heapsort#Bottom-up_heapsort),
/// [Weak heap sort](https://en.wikipedia.org/wiki/Weak_heap#Weak-heap_sort).
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
/// version of algorithm is more effective.
/// 
/// Weak-heap sort main aim is to minimize amount of comparisons between
/// elements. Amount of comparisons is basically lowered down to nearly
/// nlogn - n / ln2 + O(logn)
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
/// ```rust
/// let mut vec = vec![5, 2, 7, 3, 9];
/// sorting_rs::weak_heap_sort(&mut vec);
/// debug_assert_eq!(vec, &[2, 3, 5, 7, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::weak_heap_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn heap_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

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
    if input.len() < 2 {return;}
    
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

pub fn weak_heap_sort<T: PartialOrd>(input: &mut [T]) {
    let n = input.len();

    if n < 2 {return;}
    else {
        let mut r = vec![0; (n + 7) / 8];
        for i in (1..n).rev() {
            let mut j = i;
            while j & 1 == get_flag(&r, j >> 1) {j >>= 1;}
            let gparent = j >> 1;
            weak_heap_merge(input, &mut r, gparent, i);
        }

        for i in (2..n).rev() {
            input.swap(0, i);
            let mut x = 1;
            let mut y = 2 * x + get_flag(&r, x);
            while y < i {
                x = y;
                y = 2 * x + get_flag(&r, x);
            }
            while x > 0 {
                weak_heap_merge(input, &mut r, 0, x);
                x >>= 1;
            }
        }
        input.swap(0, 1);
    }
}

fn weak_heap_merge<T: PartialOrd>(input: &mut [T], r: &mut Vec<usize>,
i: usize, j: usize) {
    if input[i] < input[j] {
        tog_flag(r, j);
        input.swap(i, j);
    }
}

fn get_flag(r: &Vec<usize>, x: usize) -> usize {
    (r[x >> 3] >> (x & 7)) & 1
}

fn tog_flag(r: &mut Vec<usize>, x: usize) {
    r[x >> 3] ^= 1 << (x & 7)
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
    fn test_heap_empty() {
        let mut vector_in:Vec<i32> = vec![];
        heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_heap_len1() {
        let mut vector_in = vec![1];
        heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_heap_bottom_up() {
        let mut array = [10, 20, 11, 24, 22, 21, 19];
        heap_bottom_up_sort(&mut array);
        debug_assert_eq!(array, [10, 11, 19, 20, 21, 22, 24]);
    }
    #[test]
    fn test_heap_bottom_up_empty() {
        let mut vector_in:Vec<i32> = vec![];
        heap_bottom_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_heap_bottom_up_len1() {
        let mut vector_in = vec![1];
        heap_bottom_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_weak_heap_small() {
        let mut vector_in = vec![10, 20, 11, 24, 13];
        weak_heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 13, 20, 24]);
    }
    #[test]
    fn test_weak_heap_big() {
        let mut array = [10, 20, 11, 24, 22, 21, 19, 9, 7, 8, 6, 5];
        weak_heap_sort(&mut array);
        debug_assert_eq!(array, [5, 6, 7, 8, 9, 10, 11, 19, 20, 21, 22, 24]);
    }
    #[test]
    fn test_weak_heap_empty() {
        let mut vector_in:Vec<i32> = vec![];
        weak_heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_weak_heap_len1() {
        let mut vector_in = vec![1];
        weak_heap_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}