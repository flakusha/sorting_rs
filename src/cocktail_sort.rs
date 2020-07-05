/// Sorts a slice in-place using
/// [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// Cocktail sort is a variation of bubble sort. The difference is that bubble
/// sort only makes forward passes, whereas cocktail sort goes back and forth,
/// like a cocktail shaker. In practice cocktail sort is faster than bubble
/// most of the time.  A small optimization can be made where the last swap
/// position is remembered, since all elements beyond that point are already
/// sorted.
///
/// # Examples
/// ```rust
/// let mut slice = vec![2,3,4,5,1];
/// sorting_rs::cocktail_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::cocktail_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn cocktail_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let mut index_st = 0;
    let mut index_end = input.len() - 1;

    while index_st < index_end {
        let slice = index_st..index_end;
        index_end = index_st;
        for i in slice {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                index_end = i;
            }
        }
        
        let slice = (index_st..index_end).rev();
        index_st = index_end;
        for i in slice {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                index_st = i;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail() {
        let mut vector_in = vec![30, 10, 20, 11, 24, 44, 12, 11];
        cocktail_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 11, 12, 20, 24, 30, 44]);
    }
}