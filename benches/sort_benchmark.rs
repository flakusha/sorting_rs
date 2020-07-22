// #[macro_use]
use criterion::{
    criterion_group, criterion_main,
    Criterion, Bencher, ParameterizedBenchmark
};
use rand::prelude::*;

fn get_random_vec(n: usize) -> Vec<usize> {
    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let mut vec: Vec<usize> = (0..n).collect();
    vec.shuffle(&mut rng);
    vec
}

macro_rules! create_bench_function {
    ($x:ident) => {
        |b: &mut Bencher, n: &usize| {
            let s = get_random_vec(*n);
            b.iter(|| sorting_rs::$x(&mut s.clone()));
        }
    };
}

macro_rules! create_bench {
    ($p: expr, $f: ident, $($s: ident), *) => {
        ParameterizedBenchmark::new(stringify!($f),
        create_bench_function!($f), $p)
    $(.with_function(stringify!($s), create_bench_function!($s)))*
    }
}

fn bench(c: &mut Criterion) {
    let sizes: Vec<usize> = vec![10, 100, 1000, 10_000, /*100_000, 1_000_000,
    10_000_000*/];

    let benchmark = create_bench! {
        sizes,
        bubble_sort,
        cocktail_sort,
        comb_sort,
        gnome_sort,
        heap_sort,
        heap_bottom_up_sort,
        insertion_sort,
        merge_sort,
        nheap_sort,
        oddeven_sort,
        quick_sort,
        selection_sort,
        shell_sort,
        smooth_sort
        // Exclude extremely slow sorts
        // slow_sort,
        // stooge_sort
    };

    c.bench("sort_bench", benchmark);
}

criterion_group!(benches, bench);
criterion_main!(benches);