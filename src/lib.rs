//! # This library contains following sorting algorithms:
//! 
//! | Sorting algorithm | Features and downsides | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O() |
//! | -------------- | -------------------------------- | -------- | -------- | ------------- |
//! | Bubble | bad for sorted or reversed input | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
//! | Cocktail | little performance improvement over bubble sort | `n`<sup>`2`</sup> | `n` | `1` |
//! | Comb | speeds up when data is nearly sorted | `n`<sup>`2`</sup> | `nlogn` | `1` |
//! | Gnome | simple and slow, works with one item at a time | `n`<sup>`2`</sup> | `n` | `1` |
//! | Heap | independent of data distribution | `nlogn` | `n log n` or `n` | `n` or `1` |
//! | Insertion | simple, but less effective than quicksort, heapsort or merge sort | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
//! | Merge | independent of data distribution | `nlogn` | `nlogn` or `n` | `n` |
//! | Quick | bad for sorted or reversed input | `n`<sup>`2`</sup> | `n` | `n` or `logn` |
//! | Selection | the least number of swaps among all the algorithms | `n`<sup>`2`</sup>; `n` | `n`<sup>`2`</sup>; `1` | `1` |
//! | Shell | it is optimization of insertion sort | `n`<sup>`2`</sup> or `nlog`<sup>`2`</sup>`n` | `nlogn` or `nlog`<sup>`2`</sup>`n` | `n` |

pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod gnome_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::cocktail_sort::cocktail_sort;
pub use self::comb_sort::comb_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;