/// Sorts a slice in-place using
/// [Quick sort](https://en.wikipedia.org/wiki/Quicksort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// Quicksort can be compared to merge sort as it also is a divide-and-conquer
/// algorithm. However, quicksort does all the heavy work before the recursive
/// calls, so it could also be called a conquer-and-divide algorithm. This
/// implementation uses the [Lomuto partition scheme](https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme).
///
/// # Examples
/// ```rust
/// let mut vec = vec![0, -1, -2, -3,];
/// sorting_rs::quick_sort(&mut vec);
/// assert_eq!(vec, &[-3, -2, -1, 0]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::quick_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn quick_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() > 1 {
        let pivot = lomuto_partition(input);
        quick_sort(&mut input[..pivot]);
        quick_sort(&mut input[pivot + 1..]);
    }
}

/// Partitions a slice according to the Lomuta partition scheme.
pub fn lomuto_partition<T: PartialOrd>(input: &mut [T]) -> usize {
    let pivot = input.len() - 1;
    let mut swap = 0;
    for i in 0..pivot {
        if input[i] < input[pivot] {
            if swap != i {
                input.swap(swap, i);
            }
            swap += 1;
        }
    }

    if swap != pivot {
        input.swap(swap, pivot);
    }
    swap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick() {
        let mut vector_in = vec![10, 20, 11, 24];
        quick_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
}