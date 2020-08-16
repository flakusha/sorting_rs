/// Sorts a slice in-place using
/// [Bitonic sort](https://en.wikipedia.org/wiki/Bitonic_sorter).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Bitonic sort is one of the fastest sorting networks. Sorting network has the
/// sequence of comparisons that are not data-dependent.
/// 
/// Default implementations of this algorithm demand power of two elements in
/// array, but for API consistency any length is supported in case of this
/// implementation. This flexibility comes at cost though: the least effective
/// operation is `Vec::drain(..index)`, which removes additional values from the
/// beginning of Vec. Also algotithm has to place new <T> instances to make
/// array compatible with logic.
/// 
/// In the current implementation maximum supported array length is
/// `9223372036854775808`. Next power of two which is `usize::MAX` is actually
/// 2<sup>64</sup>-1, but isn't supported in most of known cases anyway, as it
/// occupies 147.6 exabytes of memory.
/// 
/// Performance-wise all the available 64 bit powers of two are calculated and
/// placed into const.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::bitonic_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::bitonic_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn bitonic_sort<T: PartialOrd + Default + Clone>(input: &mut Vec<T>) {
    if input.len() < 2 {return;}
    else if input.len() > 9223372036854775808 {panic!("Array is too big")}
    
    let in_len = input.len();

    // Check if input array has length of power of 2 and add None to fill it up
    let mut add_len = 0;
    println!("{}", add_len);

    for (i, num) in crate::POWERS_OF_TWO.iter().enumerate() {
        if in_len == *num {add_len = 0;}
        if i > 0 {
            if in_len < *num {if in_len > crate::POWERS_OF_TWO[i - 1] {
                add_len = num - in_len;}
            }
        }
    }

    if add_len > 0 {
        input.append(&mut vec![T::default(); add_len]);
    }
    
    bit_sort(input, true);
    input.drain(..add_len);
}


fn bit_sort<T: PartialOrd>(input: &mut [T], mode: bool) {
    if input.len() > 1 {
        let mid_point = input.len() / 2;
        bit_sort(&mut input[..mid_point], true);
        bit_sort(&mut input[mid_point..], false);
        sub_sort(input, mode);
    }
}
fn sub_sort<T: PartialOrd>(input: &mut [T], mode: bool) {
    if input.len() > 1 {
        compare_and_swap(input, mode);
        let mid_point = input.len() / 2;
        sub_sort(&mut input[..mid_point], mode);
        sub_sort(&mut input[mid_point..], mode);
    }
}
fn compare_and_swap<T: PartialOrd>(input: &mut [T], mode: bool) {
    let mid_point = input.len() / 2;
    for i in 0..mid_point {
        if (input[i] > input[mid_point + i]) == mode {
            input.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_usize() {
        let mut vector_in = vec![10, 20, 11, 24, 15];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 15, 20, 24]);
    }
    #[test]
    fn test_bitonic_usize_pow_2() {
        let mut vector_in = vec![10, 20, 11, 24, 15];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 15, 20, 24]);
    }
    #[test]
    fn test_bitonic_bool() {
        let mut vector_in = vec![false, true, false, false, true];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![false, false, false, true, true]);
    }
    #[test]
    fn test_bitonic_char() {
        let mut vector_in = vec!['r', 'u', 's', 't', 'c'];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec!['c', 'r', 's', 't', 'u']);
    }
    #[test]
    fn test_bitonic_empty() {
        let mut vector_in:Vec<u8> = vec![];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![]);
    }
    #[test]
    fn test_bitonic_len1() {
        let mut vector_in = vec![1];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}