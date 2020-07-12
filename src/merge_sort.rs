/// Sorts a slice out-of-place using
/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut vector_in = vec![10, 20, 11, 24];
        merge_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
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
}