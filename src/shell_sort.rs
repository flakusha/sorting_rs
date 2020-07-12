/// Sorts a slice in-place using
/// [Shell sort](https://en.wikipedia.org/wiki/Shellsort).
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// Shellsort is an optimization of insertion sort that allows the exchange of
/// items that are far apart. The idea is to arrange the list of elements so
/// that, starting anywhere, taking every hth element produces a sorted list.
/// Such a list is said to be h-sorted. It can also be thought of as h
/// interleaved lists, each individually sorted.
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![0, -1, -2, -3,];
/// sorting_rs::shell_sort(&mut vec);
/// assert_eq!(vec, &[-3, -2, -1, 0]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::shell_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

struct GapSequence {
    gap: usize,
}

impl GapSequence {
    fn new(n: usize) -> Self {
        Self { gap: n }
    }
}

impl Iterator for GapSequence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.gap /= 2;

        if self.gap > 0 {
            Some(self.gap)
        } else {
            None
        }
    }
}

pub fn shell_sort<T: PartialOrd>(input: &mut [T]) {
    if input.len() < 2 {return;}
    
    let len = input.len();
    let gaps = GapSequence::new(len);

    for gap in gaps {
        for i in gap..len {
            let mut j = i;

            while j >= gap && input[j - gap] > input[j] {
                input.swap(j - gap, j);

                j -= gap;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_gap_sequence() {
        let gaps: Vec<_> = GapSequence::new(10).collect();
        debug_assert_eq!(gaps, &[5, 2, 1]);
    }
    #[test]
    fn test_shell() {
        let mut vector_in = vec![11, 20, 21, 40, 11, 60, 5];
        shell_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![5, 11, 11, 20, 21, 40, 60]);
    }
    #[test]
    fn test_shell_empty() {
        let mut vector_in:Vec<i32> = vec![];
        shell_sort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_shell_len1() {
        let mut vector_in = vec![1];
        shell_sort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}
