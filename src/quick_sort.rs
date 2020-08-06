/// Sorts a slice in-place using
/// [Quick sort](https://en.wikipedia.org/wiki/Quicksort), 
/// [Dual-Pivot Quicksort](https://www.researchgate.net/publication/259264490_Dual_pivot_Quicksort)
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// Dual pivot quicksort additionally needs [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
/// 
/// Quicksort can be compared to merge sort as it also is a divide-and-conquer
/// algorithm. However, quicksort does all the heavy work before the recursive
/// calls, so it could also be called a conquer-and-divide algorithm. This
/// implementation uses the
/// [Lomuto partition scheme](https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme).
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
/// ```rust
/// let mut vec = vec![0, -1, -2, -3,];
/// sorting_rs::quick_dual_sort(&mut vec);
/// assert_eq!(vec, &[-3, -2, -1, 0]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::quick_dual_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn quick_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() > 1 {
        let pivot = lomuto_partition(input);
        quick_sort(&mut input[..pivot]);
        quick_sort(&mut input[pivot + 1..]);
    }
}

/// Partitions a slice according to the Lomuto partition scheme.
fn lomuto_partition<T: PartialOrd>(input: &mut [T]) -> usize {
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

pub fn quick_dual_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    dual_pivot(input, 0, input.len() - 1);
}

fn dual_pivot<T: PartialOrd + Copy>(input: &mut [T], start: usize,
end: usize) {
    if start >= end {return;}
    if input[start] > input[end] {
        input.swap(start, end);
    }
    let lpivot = input[start];
    let rpivot = input[end];

    let mut startm = start + 1;
    let mut endm = end - 1;

    let mut point = startm;

    while point <= endm {
        if input[point] < lpivot {
            input.swap(point, startm);
            startm += 1;
        }
        else if input[point] >= rpivot {
            while input[endm] > rpivot && point < endm {
                endm -= 1;
            }
            input.swap(point, endm);

            if input[point] < lpivot {
                input.swap(point, startm);
                startm += 1;
            }
        }
        point += 1;
    }
    startm -= 1;
    endm += 1;
    input.swap(start, startm);
    input.swap(end, endm);

    dual_pivot(input, start, startm);
    dual_pivot(input, startm + 1, endm);
    dual_pivot(input, endm, end);
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
    #[test]
    fn test_quick_empty() {
        let mut vector_in:Vec<i32> = vec![];
        quick_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_quick_len1() {
        let mut vector_in = vec![1];
        quick_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_quick_dual() {
        let mut vector_in = vec![10, 20, 11, 24];
        quick_dual_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_quick_two_elem() {
        let mut vector_in = [20, 10];
        quick_dual_sort(&mut vector_in);
        debug_assert_eq!(vector_in, [10, 20]);
    }
    #[test]
    fn test_quick_dual_longer() {
        let mut vector_in = [10, 20, 11, 24, 22, 21, 19];
        quick_dual_sort(&mut vector_in);
        debug_assert_eq!(vector_in, [10, 11, 19, 20, 21, 22, 24]);
    }
    #[test]
    fn test_quick_dual_empty() {
        let mut vector_in:Vec<i32> = vec![];
        quick_dual_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_quick_dual_len1() {
        let mut vector_in = vec![1];
        quick_dual_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}