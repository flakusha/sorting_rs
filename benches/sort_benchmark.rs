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
    let sizes: Vec<usize> = vec![
        2, 5, 8, 10, 16, 100, 128, 500, 512, 1000, 1024, 2000, 2048, 8000, 8192,
        10_000, 16_000, 16_384, 32_000, 32_768, 50_000, /*65_000, 65_536,*/
        100_000, /* 131_000, 131_072, 262_000, 262_144,*/ 500_000, /*524_288,*/
        1_000_000, /*1_048_576,*/ 2_000_000, /*2_097_152, 8_388_608,*/
        10_000_000, /*15_000_000, 16_777_216, 20_000_000,*/ 50_000_000,
        100_000_000, 250_000_000
    ];

    let benchmark = create_bench! {
        sizes,
        bingo_sort,
        bitonic_sort,
        bubble_sort,
        cocktail_sort,
        comb_sort,
        cycle_sort,
        gnome_sort,
        gnome_up_sort,
        heap_sort,
        heap_bottom_up_sort,
        weak_heap_sort,
        insertion_sort,
        ksort,
        merge_sort,
        merge_bottom_up_sort,
        nheap_sort,
        oddeven_sort,
        pancake_sort,
        quick_sort,
        quick_dual_sort,
        selection_sort,
        selection_double_sort,
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