/// Sorts a slice out-of-place using
/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort) and
/// [Bottom Up Merge sort](https://en.wikipedia.org/wiki/Merge_sort#Bottom-up_implementation)
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// # Examples
/// ```rust
/// let mut slice = vec![3,2,1,4];
/// sorting_rs::merge_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::merge_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
/// ```rust
/// let mut slice = vec![3,2,1,4];
/// sorting_rs::merge_bottom_up_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::merge_bottom_up_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
use std::cmp::min;

pub fn merge_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let len = input.len();
    let mid = len / 2;
    merge_sort(&mut input[..mid]);
    merge_sort(&mut input[mid..]);

    let mut tmp = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        if input[i] < input[j] {
            tmp.push(input[i]);
            i += 1;
        } else {
            tmp.push(input[j]);
            j += 1;
        }
    }
    if i < mid {
        tmp.extend_from_slice(&input[i..mid]);
    } else if j < len {
        tmp.extend_from_slice(&input[j..len]);
    }

    input.copy_from_slice(&tmp[..]);
}

pub fn merge_bottom_up_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    let mut width = 1;

    let mut tmp = input.to_vec();
    let len = input.len();

    while width < len {
        let mut i = 0;
        while i < len {
            let start = min(i + 2 * width, len);
            let mid = min(i + width, len);

            merge(&input[i..mid], &input[mid..start], &mut tmp[i..start]);
            input[i..start].copy_from_slice(&tmp[i..start]);

            i += 2 * width;
        }
        width *= 2;
    }
}

fn merge<T: PartialOrd + Copy>(in1: &[T], in2: &[T], tmp: &mut [T]) {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while left < in1.len() && right < in2.len() {
        if in1[left] <= in2[right] {
            tmp[index] = in1[left];
            index += 1;
            left += 1;
        } else {
            tmp[index] = in2[right];
            index += 1;
            right += 1;
        }
    }

    if left < in1.len() {
        tmp[index..].copy_from_slice(&in1[left..]);
    }
    if right < in2.len() {
        tmp[index..].copy_from_slice(&in2[right..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut vector_in = vec![10, 20, 11, 13, 24];
        merge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 13, 20, 24]);
    }
    #[test]
    fn test_merge_empty() {
        let mut vector_in:Vec<i32> = vec![];
        merge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_merge_len1() {
        let mut vector_in = vec![1];
        merge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_merge_bu() {
        let mut vector_in = vec![24, 20, 11, 13, 10];
        merge_bottom_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 13, 20, 24]);
    }
    #[test]
    fn test_merge_bu_empty() {
        let mut vector_in:Vec<i32> = vec![];
        merge_bottom_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_merge_bu_len1() {
        let mut vector_in = vec![1];
        merge_bottom_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}