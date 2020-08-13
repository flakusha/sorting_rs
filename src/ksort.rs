/// Sorts a slice in-place using
/// [K-sort](https://arxiv.org/abs/1107.3622)
/// 
/// This sorting algorithm is in fact a modification of quick sort which should
/// be faster than quicksort for arrays with less than 7 million elements
/// This algorithm is generally compared to heapsort and quicksort, it doesn't
/// need to construct the heap and generally wins in benchmarks because of this.
/// 
/// # Examples
/// ```rust
/// let mut vec = vec![5, 2, 7, 3, 9];
/// sorting_rs::ksort(&mut vec);
/// debug_assert_eq!(vec, &[2, 3, 5, 7, 9]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::ksort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn ksort<T: PartialOrd + Clone + Copy>(input: &mut [T]) {
    if input.len() < 2 {return;}
    ksort_lr(input, 0, input.len() - 1);
}

fn ksort_lr<T: PartialOrd + Clone + Copy>(input: &mut [T], left: usize,
right: usize) {
    let key = input[left].clone();
    // just init it, so no unsafe calls needed, otherwise use of uninit
    // is prohibited by Rust compiler
    let mut temp = key.clone();
    let mut i = left;
    let mut j = right + 1;
    let mut k = i + 1;
    let mut p = i + 1;
    let mut flag = false;

    while j - i >= 2 {
        if key <= input[p] {
            if p != j && j != right + 1 {
                input.swap(j, p);
            } else if j == right + 1 {
                flag = true;
                temp = input[p].clone();
            }
            j -= 1;
            p = j;
        } else {
            input.swap(i, p);
            i += 1;
            k += 1;
            p = k;
        }
    }

    input[i] = key;
    if flag {input[i + 1] = temp;}

    if left < i.saturating_sub(1) {
        ksort_lr(input, left, i - 1);
    }
    if right > i + 1 {
        ksort_lr(input, i + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ksort() {
        let mut vector_in = [10, 20, 11, 24];
        ksort(&mut vector_in);
        debug_assert_eq!(vector_in, [10, 11, 20, 24]);
    }
    #[test]
    fn test_ksort_01() {
        let mut vector_in = [10, 9, 20, 22, 11, 21, 12, 24, 4, 6, 3];
        ksort(&mut vector_in);
        debug_assert_eq!(vector_in, [3, 4, 6, 9, 10, 11, 12, 20, 21, 22, 24]);
    }
    #[test]
    fn test_ksort_empty() {
        let mut vector_in:Vec<i32> = vec![];
        ksort(&mut vector_in);
        debug_assert_eq!(vector_in, &[]);
    }
    #[test]
    fn test_ksort_len1() {
        let mut vector_in = vec![1];
        ksort(&mut vector_in);
        debug_assert_eq!(vector_in, vec![1]);
    }
}