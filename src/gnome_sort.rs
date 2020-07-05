/// Sorts a slice in-place using
/// [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Gnome sort is also known as stupid sort, because it is very easy to
/// understand and not very efficient. It is based on how a gnome would sort
/// flower pots.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::gnome_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::shell_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn gnome_sort<T: PartialOrd>(input: &mut [T]) {
    let mut i = 0;
    
    while i < input.len() {
        if i == 0 || input[i - 1] < input[i] {
            i += 1;
        } else {
            input.swap(i - 1, i);
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnome() {
        let mut vector_in = vec![10, 20, 11, 24];
        gnome_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
}