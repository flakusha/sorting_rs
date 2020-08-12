/// Sorts a slice in-place using
/// [Bitonic sort](https://en.wikipedia.org/wiki/Bitonic_sorter).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Bitonic sort is one of the fastest sorting networks. Sorting network has the
/// sequence of comparisons that are not data-dependent.
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
// use std::fmt::Debug;

pub fn bitonic_sort<T: PartialOrd + Max<T>>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let in_len = input.len();
    // Convert to vec of options
    // let mut input_m: Vec<Option<&T>> = input.into_iter().map(Some).collect();
    // Check if input array has length of power of 2 and add None to fill it up
    let mut add_len = 0;
    println!("{}", add_len);

    for (i, num) in POWERS_OF_TWO.into_iter().enumerate() {
        if in_len == *num {add_len = 0;}
        if i > 0 {
            if in_len < *num {if in_len > POWERS_OF_TWO[i - 1] {
                add_len = num - in_len;}
            }
        }
    }

    if add_len > 0 {input.append(vec![T::MAX; add_len]);}
    
    sort(&mut input, true);

}


// pub fn sort<T: PartialOrd>(input: &mut [T], mode: bool) {
//     if input.len() > 1 {
//         let mid_point = input.len() / 2;
//         sort(&mut input[..mid_point], true);
//         sort(&mut input[mid_point..], false);
//         sub_sort(input, mode);
//     }
// }
// fn sub_sort<T: PartialOrd>(input: &mut [T], mode: bool) {
//     if input.len() > 1 {
//         compare_and_swap(input, mode);
//         let mid_point = input.len() / 2;
//         sub_sort(&mut input[..mid_point], mode);
//         sub_sort(&mut input[mid_point..], mode);
//     }
// }
// fn compare_and_swap<T: PartialOrd>(input: &mut [T], mode: bool) {
//     let mid_point = input.len() / 2;
//     for i in 0..mid_point {
//         if input[i].is_none() || input[i + 1].is_none() {break;}
//         else if (input[i] > input[mid_point + i]) == mode {
//             input.swap(i, mid_point + i);
//         }
//     }
// }

// fn bit_comp<T: PartialOrd>(input: &mut Vec<T>, mode: bool) {
//     let d = input.len() / 2;
//     for i in 0..d {
//         if (input[i] > input[i + d]) == mode {
//             input.swap(i, i + d);
//         }
//     }
// }

/// usize for 64 bit is limited with 63 numbers
const POWERS_OF_TWO: [usize; 63] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024,
2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576,
2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456,
536870912, 1073741824, 2147483648, 4294967296, 8589934592, 17179869184,
34359738368, 68719476736, 137438953472, 274877906944, 549755813888,
1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416,
35184372088832, 70368744177664, 140737488355328, 281474976710656,
562949953421312, 1125899906842624, 2251799813685248, 4503599627370496,
9007199254740992, 18014398509481984, 36028797018963968, 72057594037927936,
144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976,
2305843009213693952, 4611686018427387904, 9223372036854775808];

/// Implemented Max values for generics to use in the code
macro_rules! impl_max {
    ($($t:ty)*) => ($(
        impl Max for $t {
            #[inline]
            fn get_max(&self) -> Self {*self::MAX}
        }
    )*)
}
pub trait Max<T: PartialOrd> {fn get_max(&self) -> Self;}
// impl Max<T> for T {fn get_max(&self) -> T {T::MAX}}
// impl Max<u8> for u8 {fn get_max(&self) -> u8 {u8::MAX}}
impl_max! {
    bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
}


// fn bit_sort<T: PartialOrd>(input: &mut Vec<T>, mode: bool) {
//     if input.len() < 2 {return;}
    
//     let mut left: Vec<T> = input.drain(..input.len() / 2).collect();
//     bit_sort(&mut left, true);
//     bit_sort(input, false);
//     left.append(input);
//     bit_merge(&mut left, mode);
//     std::mem::swap(input, &mut left);
// }

// fn bit_merge<T: PartialOrd>(input: &mut Vec<T>, mode) {
//     if input.len() == 1 {return;}
//     else {
//         bit_comp(input, mode);
//         let mut left: Vec<T> = input.drain(..input.len() / 2).collect();
//         bit_merge(mode, &mut left);
//         bit_merge(mode, input);
//         left.append(input);
//         std::mem::swap(input, &mut left);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic() {
        let mut vector_in = vec![10, 20, 11, 24, 15];
        bitonic_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 15, 20, 24]);
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
    #[test]
    fn test_u8_max() {

        let mut input = 12;
        input = input.get_max();
        debug_assert_eq!(input, u8::MAX);
    }
}