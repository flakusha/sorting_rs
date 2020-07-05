# rust_sorting
# This library contains following sorting algorithms implemented in Rust:
 
| Sorting algorithm | Features and downsides | Worst-case performance O(): comparisons; swaps | Best-case performance O(): comparisons; swaps | Space complexity O() |
| -------------- | -------------------------------- | -------- | -------- | ------------- |
| Bubble | bad for sorted or reversed input | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
| Cocktail | little performance improvement over bubble sort | `n`<sup>`2`</sup> | `n` | `1` |
| Comb | speeds up when data is nearly sorted | `n`<sup>`2`</sup> | `n log n` | `1` |
| Gnome | simple and slow, works with one item at a time | `n`<sup>`2`</sup> | `n` | `1` |
| Heap | independent of data distribution | `nlogn` | `nlogn` or `n` | `n` or `1` |
| Insertion | simple, but less effective than quicksort, heapsort or merge sort | `n`<sup>`2`</sup>; `n`<sup>`2`</sup> | `n`; `1` | `n` or `1` |
| Merge | independent of data distribution | `nlogn` | `nlogn` or `n` | `n` |
| Quick | bad for sorted or reversed input | `n`<sup>`2`</sup> | `n` | `n` or `logn` |
| Selection | the least number of swaps among all the algorithms | `n`<sup>`2`</sup>; `n` | `n`<sup>`2`</sup>; `1` | `1` |
| Shell | it is optimization of insertion sort | `n`<sup>`2`</sup> or `nlog`<sup>`2`</sup>`n` | `nlogn` or `nlog`<sup>`2`</sup>`n` | `n` |