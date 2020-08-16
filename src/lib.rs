//! # This library contains following sorting algorithms:
//! 
//! | Sorting algorithm | Features and downsides                                               | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O()   |
//! | ----------------- | -------------------------------------------------------------------- | ---------------------------------------------- | --------------------------------------------- | ---------------------- |
//! | Bingo             | aims to be faster than selection sort if there are duplicates        | `n + m`<sup>2</sup>                            | `nm`                                          |                        |
//! | Bitonic           | method based on building a sorting network                           | `nlog`<sup>`2`</sup>`n`                        | `nlog`<sup>`2`</sup>`n`                       | `nlog`<sup>`2`</sup>`n`|
//! | Bubble            | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                    |
//! | Cocktail          | little performance improvement over bubble sort                      | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
//! | Comb              | speeds up when data is nearly sorted                                 | `n`<sup>`2`</sup>                              | `nlogn`                                       | `1`                    |
//! | Cycle             | uses minimum amount of writes, good for memory with limited TBW      | `n`<sup>`2`</sup>                              | `n`<sup>`2`</sup>                             | `1`                    |
//! | Gnome             | simple and slow, works with one item at a time                       | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
//! | Heap              | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `1`                    |
//! | Weak Heap         | independent of data distribution, decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                    |
//! | N-Heap            | should be faster than default heap. N = 3                            | `nlogn`                                        | `nlogn`                                       | `1`                    |
//! | Bottom-up Heap    | upgraded version of heapsort with decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                    |
//! | Insertion         | simple, but less effective than quicksort, heapsort or merge sort    | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                    |
//! | Merge             | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `n`                    |
//! | Merge Bottom-up   | independent of data distribution, modified version of mergesort      | `nlogn`                                        | `nlogn`                                       | `n`                    |
//! | Odd-even          | presented to be effective on processors with local interconnections  | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
//! | Odd-even Batcher  | more efficient version of odd-even sort                              | `log`<sup>`2`</sup>`n`                         | `log`<sup>`2`</sup>`n`                        | `logn`<sup>`2`</sup>   |
//! | Pancake           | swaps data a lot and not so effective in practice                    | `n`<sup>`3`</sup>; `2n - 3`                    | `n`<sup>`2`</sup>                             | `n`                    |
//! | Quick             | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>                              | `nlogn`                                       | `logn`                 |
//! | Quick dual        | enchanced version of quicksort                                       | `n`<sup>`2`</sup>                              | `2nlnn`                                       | `logn`                 |
//! | Ksort             | quicksort variant, faster than heap at less than 7 million elements  | `n`<sup>`2`</sup>                              | `nlog`<sub>2</sub>`n`                         | `logn`                 |
//! | Selection         | the least number of swaps among all the algorithms                   | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | `1`                    |
//! | Double selection  | modified selection sort with more workload, but better efficiency    | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | higher than Selection  |
//! | Shellsort         | it is optimization of insertion sort                                 | `n`<sup>`3/2`</sup> or `nlogn`<sup>`2`</sup>   | `nlogn`                                       | `1`                    |
//! | Slow              | it's slow, who would ever need it?                                   |                                                |                                               |                        |
//! | Smooth            | variant of heapsort, good for nearly sorted data                     | `nlogn`                                        | `n`                                           | `1`                    |
//! | Stooge            | it's a bit faster than slow sort                                     | `n`<sup>`2.7095`</sup>                         |                                               | `n`                    |

pub mod bingo_sort;
pub mod bitonic_sort;
pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod cycle_sort;
pub mod gnome_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod ksort;
pub mod merge_sort;
pub mod nheap_sort;
pub mod oddeven_sort;
pub mod pancake_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;
pub mod slow_sort;
pub mod smooth_sort;
pub mod stooge_sort;

pub use self::bingo_sort::bingo_sort;
pub use self::bitonic_sort::bitonic_sort;
pub use self::bubble_sort::bubble_sort;
pub use self::cocktail_sort::cocktail_sort;
pub use self::comb_sort::comb_sort;
pub use self::cycle_sort::cycle_sort;
pub use self::gnome_sort::{gnome_sort, gnome_up_sort};
pub use self::heap_sort::{heap_sort, heap_bottom_up_sort, weak_heap_sort};
pub use self::nheap_sort::nheap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::ksort::ksort;
pub use self::merge_sort::{merge_sort, merge_bottom_up_sort};
pub use self::oddeven_sort::{oddeven_sort, oddeven_batcher_sort};
pub use self::pancake_sort::pancake_sort;
pub use self::quick_sort::{quick_sort, quick_dual_sort};
pub use self::selection_sort::{selection_sort, selection_double_sort};
pub use self::shell_sort::shell_sort;
pub use self::slow_sort::slow_sort;
pub use self::smooth_sort::smooth_sort;
pub use self::stooge_sort::stooge_sort;

/// Calculated powers of 2
pub(crate) const POWERS_OF_TWO: [usize; 63] = [
2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648, 4294967296,
8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944,
549755813888, 1099511627776, 2199023255552, 4398046511104, 8796093022208,
17592186044416, 35184372088832, 70368744177664, 140737488355328,
281474976710656, 562949953421312, 1125899906842624, 2251799813685248,
4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968,
72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488,
1152921504606846976, 2305843009213693952, 4611686018427387904,
9223372036854775808
];

/// Calculated Leonardo numbers
pub(crate) const LEO_NUMS: [usize; 90] = [
    1, 1, 3, 5, 9, 15, 25, 41, 67, 109, 177, 287, 465, 753, 1219, 1973, 3193,
    5167, 8361, 13529, 21891, 35421, 57313, 92735, 150049, 242785, 392835,
    635621, 1028457, 1664079, 2692537, 4356617, 7049155, 11405773, 18454929,
    29860703, 48315633, 78176337, 126491971, 204668309, 331160281, 535828591,
    866988873, 1402817465, 2269806339, 3672623805, 5942430145, 9615053951,
    15557484097, 25172538049, 40730022147, 65902560197, 106632582345,
    172535142543, 279167724889, 451702867433, 730870592323, 1182573459757,
    1913444052081, 3096017511839, 5009461563921, 8105479075761,
    13114940639683, 21220419715445, 34335360355129, 55555780070575,
    89891140425705, 145446920496281, 235338060921987, 380784981418269,
    616123042340257, 996908023758527, 1613031066098785, 2609939089857313,
    4222970155956099, 6832909245813413, 11055879401769513, 17888788647582927,
    28944668049352441, 46833456696935369, 75778124746287811, 122611581443223181,
    198389706189510993, 321001287632734175, 519390993822245169,
    840392281454979345, 1359783275277224515, 2200175556732203861,
    3559958832009428377, 5760134388741632239,
];