/// Sorts a slice in-place using
/// [Smooth sort](https://en.wikipedia.org/wiki/Smoothsort)
/// 
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
///
/// This sorting algorithm transforms the input array into implicit heap data
/// structure and then produces the sorted array by repeatedly extracting the
/// largest remaining element
///
/// # Examples
/// ```rust
/// let mut vec = vec![5,3,2,4];
/// sorting_rs::smooth_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
/// ```rust
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorting_rs::smooth_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```

pub fn smooth_sort<T: PartialOrd>(input: &mut [T]) {
    unimplemented!("Not yet ready!");
}