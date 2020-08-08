# sorting_rs
Sorting algorithms implemented in Rust
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
3. For more information about origin of algorithms and implementation details, 
please read modules documentation.
[wikipedia](https://en.wikipedia.org/wiki/Sorting_algorithm) is nice starting
point too
4. For API 

## This library contains following sorting algorithms:
 
| Sorting algorithm | Features and downsides                                               | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O() |
| ----------------- | -------------------------------------------------------------------- | ---------------------------------------------- | --------------------------------------------- | -------------------- |
| Bubble            | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                  |
| Cocktail          | little performance improvement over bubble sort                      | `n`<sup>`2`</sup>                              | `n`                                           | `1`                  |
| Comb              | speeds up when data is nearly sorted                                 | `n`<sup>`2`</sup>                              | `nlogn`                                       | `1`                  |
| Gnome             | simple and slow, works with one item at a time                       | `n`<sup>`2`</sup>                              | `n`                                           | `1`                  |
| Heap              | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `1`                  |
| Weak Heap         | independent of data distribution, decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                  |
| N-Heap            | should be faster than default heap. n = 3                            | `nlogn`                                        | `nlogn`                                       | `1`                  |
| Bottom-up Heap    | upgraded version of heapsort with decreased number of comparisons    | `nlogn`                                        | `nlogn`                                       | `1`                  |
| Insertion         | simple, but less effective than quicksort, heapsort or merge sort    | `n`<sup>`2`</sup>; `n`<sup>`2`</sup>           | `n`; `1`                                      | `1`                  |
| Merge             | independent of data distribution                                     | `nlogn`                                        | `nlogn`                                       | `n`                  |
| Odd-even          | presented to be effective on processors with local interconnections  | `n`<sup>`2`</sup>                              | `n`                                           | `1`                  |
| Odd-even Batcher  | more efficient version of odd-even sort                              | `nlogn`<sup>`2`</sup>                          | `logn`<sup>`2`</sup>                          | `logn`<sup>`2`</sup> |
| Quick             | bad for sorted or reversed input                                     | `n`<sup>`2`</sup>                              | `nlog`<sub>2</sub>`n`                         | `logn`               |
| Quick dual        | enchanced version of quicksort                                       | `n`<sup>`2`</sup>                              | `2nlnn`                                       | `logn`               |
| Ksort             | quicksort variant, faster than heap at less than 7 million elements  | `n`<sup>`2`</sup>                              | `nlog`<sub>2</sub>`n`                         | `logn`               |
| Selection         | the least number of swaps among all the algorithms                   | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | `1`                  |
| Double selection  | modified selection sort with more workload, but better efficiency    | `n`<sup>`2`</sup>; `n`                         | `n`<sup>`2`</sup>; `1`                        | higher than Selection|
| Shellsort         | it is optimization of insertion sort                                 | `n`<sup>`3/2`</sup> or `nlogn`<sup>`2`</sup>   | `nlogn`                                       | `1`                  |
| Slow              | it's slow, who would ever need it?                                   |                                                |                                               |                      |
| Smooth            | variant of heapsort, good for nearly sorted data                     | `nlogn`                                        | `n`                                           | `1`                  |
| Stooge            | it's a bit faster than slow sort                                     | `n`<sup>`2.7095`</sup>                         |                                               | `n`                  |

New algorithms implementations are planned