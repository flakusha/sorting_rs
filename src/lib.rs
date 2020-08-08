//! # This library contains following sorting algorithms:
//! 
//! | Sorting algorithm | Features and downsides | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O() |
//! | -------------- | -------------------------------- | -------- | -------- | ------------- |
//! | Bubble | bad for sorted or reversed input | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
//! | Cocktail | little performance improvement over bubble sort | `n`<sup>`2`</sup> | `n` | `1` |
//! | Comb | speeds up when data is nearly sorted | `n`<sup>`2`</sup> | `nlogn` | `1` |
//! | Gnome | simple and slow, works with one item at a time | `n`<sup>`2`</sup> | `n` | `1` |
//! | Heap | independent of data distribution | `nlogn` | `nlogn` or `n` | `n` or `1` |
//! | Weak Heap | independent of data distribution, decreased amount of comparisons | `nlogn` | `nlogn` or `n` | `n` or `1` |
//! | N-Heap | independent of data distribution, should be faster than default heap. n = 3 | `nlogn` | `nlogn` or `n` | `n` or `1` |
//! | Bottom-up Heap | upgraded version of heapsort with decreased number of comparisons | `nlogn` | `nlogn` or `n` | `n` or `1` |
//! | Insertion | simple, but less effective than quicksort, heapsort or merge sort | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
//! | Merge | independent of data distribution | `nlogn` | `nlogn` | `n` |
//! | Odd-even | presented to be effective on processors with local interconnections | `n`<sup>`2`</sup> | `n` | `1` |
//! | Odd-even (Batcher) | more efficient version of odd-even sort | `nlogn`<sup>`2`</sup> | `logn`<sup>`2`</sup> | `logn`<sup>`2`</sup> |
//! | Quick | bad for sorted or reversed input | `n`<sup>`2`</sup> | `nlog`<sub>2</sub>`n` | `n` or `logn` |
//! | Quick dual | enchanced version of quicksort | `n`<sup>`2`</sup> | `2nlnn` | `n` or `logn` |
//! | Ksort | modified version of quicksort, faster than heap at less than 7 million elements | `n`<sup>`2`</sup> | `nlog`<sub>2</sub>`n` | `n` or `logn` |
//! | Selection | the least number of swaps among all the algorithms | `n`<sup>`2`</sup>; `n` | `n`<sup>`2`</sup>; `1` | `1` |
//! | Double selection | modified version of selection sort with more workload, but better efficiency | `n`<sup>`2`</sup>; `n` | `n`<sup>`2`</sup>; `1` | more than Selection |
//! | Shellsort | it is optimization of insertion sort | `n`<sup>`3/2`</sup> or `nlogn`<sup>`2`</sup> | `nlogn` | `1` |
//! | Slow | it's slow, who would ever need it? | | | |
//! | Smooth | variant of heapsort, good for nearly sorted data | `nlogn` | `n` | `n` or `1` |
//! | Stooge | it's a bit faster than slow sort | `n`<sup>`2.7095`</sup> | | `n` |

pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod gnome_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod ksort;
pub mod merge_sort;
pub mod nheap_sort;
pub mod oddeven_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;
pub mod slow_sort;
pub mod smooth_sort;
pub mod stooge_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::cocktail_sort::cocktail_sort;
pub use self::comb_sort::comb_sort;
pub use self::gnome_sort::{gnome_sort, gnome_up_sort};
pub use self::heap_sort::{heap_sort, heap_bottom_up_sort, weak_heap_sort};
pub use self::nheap_sort::nheap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::ksort::ksort;
pub use self::merge_sort::merge_sort;
pub use self::oddeven_sort::{oddeven_sort, oddeven_batcher_sort};
pub use self::quick_sort::{quick_sort, quick_dual_sort};
pub use self::selection_sort::{selection_sort, selection_double_sort};
pub use self::shell_sort::shell_sort;
pub use self::slow_sort::slow_sort;
pub use self::smooth_sort::smooth_sort;
pub use self::stooge_sort::stooge_sort;