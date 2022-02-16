use std::{iter, num};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use orst::get_unsorted_vector;
use orst::sort::{
    bubble_sort, heap_sort::heap_sort, merge_sort::merge_sort, quick_sort::quick_sort,
    radix_sort::radix_sort,
};

fn bench_bubble_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_bubble_sort");
    for size in [100, 1000, 10000, 100000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut arr = get_unsorted_vector(0, size);
                bubble_sort(&mut arr);
            });
        });
        group.finish();
    }
}

fn main() {
    criterion_group!(benches, bench_bubble_sort);
    criterion_main!(benches);
}
