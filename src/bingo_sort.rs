/// Sorts a slice in-place using
/// [Bingo sort](https://xlinux.nist.gov/dads/HTML/bingosort.html).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// This algorithm aims to be more effective than selection sort in cases there
/// are many duplicate values.
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::bingo_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::bingo_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn bingo_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let in_len = input.len();

    let minmax = get_min_max(input);
    let min = minmax[0];
    let max = minmax[1];

    let mut bingo = min;
    let mut n_bingo = max;
    let mut n_index = 0;

    while bingo < max {
        let start = n_index;
        for i in start..in_len {
            if input[i] == bingo {input.swap(i, n_index); n_index += 1;}
            if input[i] < n_bingo {n_bingo = input[i];}
        }
        bingo = n_bingo;
        n_bingo = max;
    }
}

fn get_min_max<T: PartialOrd + Copy>(input: &[T]) -> [T; 2] {
    let mut min = input[0];
    let mut max = input[0];
    for i in 1..input.len() {
        if input[i] < min {min = input[i];}
        if input[i] > max {max = input[i];}
    }
    [min, max]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let mut vector_in = vec![10, 20, 11, 24, 15];
        bingo_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 15, 20, 24]);
    }
    #[test]
    fn test_bingo_empty() {
        let mut vector_in:Vec<i32> = vec![];
        bingo_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![]);
    }
    #[test]
    fn test_bingo_len1() {
        let mut vector_in = vec![1];
        bingo_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}