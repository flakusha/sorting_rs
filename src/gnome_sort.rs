/// Sorts a slice in-place using
/// [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Gnome sort is also known as stupid sort, because it is very easy to
/// understand and not very efficient. It is based on how a gnome would sort
/// flower pots.
/// 
/// Upgraded version of algorithm just remember last sort position, so it
/// doesn't try to sort already sorted data.
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::gnome_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::gnome_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::gnome_up_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::gnome_up_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn gnome_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
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

pub fn gnome_up_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}

    let mut i = 1;
    let mut j = 2;
    let in_len = input.len();

    while i < in_len {
        if input[i - 1] <= input[i] {
            i = j;
            j += 1;
        }
        else {
            input.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
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
    #[test]
    fn test_gnome_empty() {
        let mut vector_in:Vec<i32> = vec![];
        gnome_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_gnome_len1() {
        let mut vector_in = vec![1];
        gnome_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
    #[test]
    fn test_gnome_up() {
        let mut vector_in = vec![10, 20, 11, 24];
        gnome_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![10, 11, 20, 24]);
    }
    #[test]
    fn test_gnome_up_empty() {
        let mut vector_in:Vec<i32> = vec![];
        gnome_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_gnome_up_len1() {
        let mut vector_in = vec![1];
        gnome_up_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}