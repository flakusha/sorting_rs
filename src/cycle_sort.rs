/// Sorts a slice in-place using
/// [Cycle sort](https://en.wikipedia.org/wiki/Cycle_sort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// 
/// This kind of sort is pretty good in case you need as low writes as
/// possible, for example, using SSD or NAND memory.
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![56, 32, 78, 16];
/// sorting_rs::cycle_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::cycle_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn cycle_sort<T: PartialOrd + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let in_len = input.len();
    for index in 0..in_len {
        let mut key = input[index];
        let mut pos = index;

        for i in index + 1..in_len {
            if input[i] < key {pos += 1;}
        }

        if pos == index {continue;}

        while key == input[pos] {
            pos += 1;
        }
        // One does not simply swap by indexes, but copies item into buffer
        let tmp = input[pos];
        input[pos] = key;
        key = tmp;

        while pos != index {
            pos = index;
            for i in index + 1..in_len {
                if input[i] < key {pos += 1;}
            }
            while key == input[pos] {
                pos += 1;
            }
            let tmp = input[pos];
            input[pos] = key;
            key = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut vector_in = vec![11, 20, 21, 40, 11, 60, 5];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![5, 11, 11, 20, 21, 40, 60]);
    }
    #[test]
    fn test_cycle_empty() {
        let mut vector_in:Vec<i32> = vec![];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_cycle_len1() {
        let mut vector_in = vec![1];
        cycle_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}