# sorting_rs
Sorting algorithms implemented in Rust.
## Usage
1. Add this dependency and please consider it's version into your Cargo.toml:
```toml
[dependencies]
sorting_rs = "1.2.0"
```
2. Use available sorting algorithms in your Rust code:
```rust
use sorting_rs::*;
```
3. API for every sorting function is pretty the same: you just have to pass
mutable reference: `&mut [T]`, or `vec![T, T, T, ...]`. `T` should have
`PartialOrd` trait, sometimes you may need `Copy` or `Clone` traits, though
all implementations try to avoid this kind of additional requirements.
4. For more information about origin of algorithms and implementation details, 
please read modules documentation.
[Wikipedia](https://en.wikipedia.org/wiki/Sorting_algorithm) is nice starting
point too.

## This library contains following sorting algorithms:
 
| Sorting algorithm | Features and downsides                                               | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O()   |
| ----------------- | -------------------------------------------------------------------- | ---------------------------------------------- | --------------------------------------------- | ---------------------- |
| Bingo             | aims to be faster than selection sort if there are duplicates        | `n + m`<sup>2</sup>                            | `nm`                                          |                        |
| Bitonic           | method based on building a sorting network                           | `nlog`<sup>`2`</sup>`n`                        | `nlog`<sup>`2`</sup>`n`                       | `nlog`<sup>`2`</sup>`n`|
| Bubble            | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                    |
| Cocktail          | little performance improvement over bubble sort                      | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
| Comb              | speeds up when data is nearly sorted                                 | `n`<sup>`2`</sup>                              | `nlogn`                                       | `1`                    |
| Cycle             | uses minimum amount of writes, good for memory with limited TBW      | `n`<sup>`2`</sup>                              | `n`<sup>`2`</sup>                             | `1`                    |
| Gnome             | simple and slow, works with one item at a time                       | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
| Heap              | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `1`                    |
| Weak Heap         | independent of data distribution, decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                    |
| N-Heap            | should be faster than default heap. N = 3                            | `nlogn`                                        | `nlogn`                                       | `1`                    |
| Bottom-up Heap    | upgraded version of heapsort with decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                    |
| Insertion         | simple, but less effective than quicksort, heapsort or merge sort    | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                    |
| Merge             | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `n`                    |
| Odd-even          | presented to be effective on processors with local interconnections  | `n`<sup>`2`</sup>                              | `n`                                           | `1`                    |
| Odd-even Batcher  | more efficient version of odd-even sort                              | `log`<sup>`2`</sup>`n`                         | `log`<sup>`2`</sup>`n`                        | `logn`<sup>`2`</sup>   |
| Pancake           | swaps data a lot and not so effective in practice                    | `n`<sup>`3`</sup>; `2n - 3`                    | `n`<sup>`2`</sup>                             | `n`                    |
| Quick             | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>                              | `nlogn`                                       | `logn`                 |
| Quick dual        | enchanced version of quicksort                                       | `n`<sup>`2`</sup>                              | `2nlnn`                                       | `logn`                 |
| Ksort             | quicksort variant, faster than heap at less than 7 million elements  | `n`<sup>`2`</sup>                              | `nlog`<sub>2</sub>`n`                         | `logn`                 |
| Selection         | the least number of swaps among all the algorithms                   | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | `1`                    |
| Double selection  | modified selection sort with more workload, but better efficiency    | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | higher than Selection  |
| Shellsort         | it is optimization of insertion sort                                 | `n`<sup>`3/2`</sup> or `nlogn`<sup>`2`</sup>   | `nlogn`                                       | `1`                    |
| Slow              | it's slow, who would ever need it?                                   |                                                |                                               |                        |
| Smooth            | variant of heapsort, good for nearly sorted data                     | `nlogn`                                        | `n`                                           | `1`                    |
| Stooge            | it's a bit faster than slow sort                                     | `n`<sup>`2.7095`</sup>                         |                                               | `n`                    |

New algorithms implementations are planned in future