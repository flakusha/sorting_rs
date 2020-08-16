/// Sorts a slice in-place using
/// [Smooth sort](https://en.wikipedia.org/wiki/Smoothsort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// This sorting algorithm transforms the input array into implicit heap data
/// structure and then produces the sorted array by repeatedly extracting the
/// largest remaining element.
/// This algorithm makes use of Leonardo numbers. It's a sequence of numbers
/// defined by:
/// ```text
/// L(0) = 1
/// L(1) = 1
/// L(n) = L(n - 1) + L(n - 2) + 1
/// *OR*
/// L(n) = 2 * Fib(n + 1) - 1
/// ```
/// Where *+ 1* is "add" number and "Fib" are Fibonacci numbers.
/// 
/// For 64-bit systems it's possible to use 90 Leonardo numbers placed as a
/// constant array [usize; 90].
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::smooth_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::smooth_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn smooth_sort<T: PartialOrd>(input: &mut [T])
{
    if input.len() < 2 {return;}
    
    // Init addtitional index heap
    let input = input;
    let in_len = input.len();
    let mut heap = Vec::<usize>::new();

    for i in 0..in_len {
        if heap.len() >= 2 && heap[heap.len() - 2] == heap[heap.len() - 1] + 1 {
            heap.pop();
            let len_leo = heap.len();
            heap[len_leo - 1] += 1;
        } else if heap.len() >= 1 && heap[heap.len() - 1] == 1 {
            heap.push(0);
        } else {
            heap.push(1);
        }
        restore_heap(input, i, &heap);
    }

    for i in (0..in_len).rev() {
        if heap[heap.len() - 1] < 2 {
            heap.pop();
        } else {
            let k = heap.pop().unwrap();
            let t = get_child_trees(i, k);
            // tr kr tl kl
            // 0  1  2  3
            heap.push(t[3]);
            restore_heap(input, t[2], &heap);
            heap.push(t[1]);
            restore_heap(input, t[0], &heap);
        }
    }
}

fn restore_heap<T: PartialOrd>(input: &mut [T], index: usize, heap: &Vec<usize>)
{
    // Insertion sorting
    let mut current = heap.len() - 1;
    let mut i = index;
    let mut k = heap[current];

    while current > 0 {
        let j = i - crate::LEO_NUMS[k];
        if input[j] > input[i] &&
        (k < 2 || input[j] > input[i - 1] && input[j] > input[i - 2]) {
            input.swap(i, j);
            i = j;
            current -= 1;
            k = heap[current];
        } else {
            break;
        }
    }

    while k >= 2 {
        let t = get_child_trees(i, k);
        // tr kr tl kl
        // 0  1  2  3
        if input[i] < input[t[0]] || input[i] < input[t[2]] {
            if input[t[0]] > input[t[2]] {
                input.swap(i, t[0]);
                i = t[0];
                k = t[1];
            } else {
                input.swap(i, t[2]);
                i = t[2];
                k = t[3];
            }
        } else {
            break;
        }
    }
}

fn get_child_trees(i: usize, k: usize) -> [usize; 4] {
    let tr = i - 1;
    let kr = k - 2;
    let tl = tr - crate::LEO_NUMS[kr];
    let kl = k - 1;
    [tr, kr, tl, kl]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth() {
        let mut vector_in = vec![20, 10, 11, 13];
        smooth_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[10, 11, 13, 20]);
    }
    #[test]
    fn test_smooth_01() {
        let mut vector_in = vec![20, 10, 11, 13, 24, 9, 2, 1, 8];
        smooth_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[1, 2, 8, 9, 10, 11, 13, 20, 24]);
    }
    #[test]
    fn test_smooth_empty() {
        let mut vector_in:Vec<i32> = vec![];
        smooth_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_smooth_len1() {
        let mut vector_in = vec![1];
        smooth_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[1]);
    }
}