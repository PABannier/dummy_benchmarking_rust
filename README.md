# A dummy repo to learn Rust benchmarking

A dummy repo to learn how to effectively benchmark in Rust. As usual,
sorting algorithms are of great help to set things up and quicky prototype.

A quick results on benchmarking for a vector of 1000 integers to sort:

- Bubble sort: `174,821 ns/iter (+/- 27,844)` `O(n²)` (Arghhh...)
- Merge sort: `81,875 ns/iter (+/- 13,887)` `O(nLogn)` (Youpi!)
- Heap sort: `52,516 ns/iter (+/- 5,491)` `O(nLogn)` (Waouhhhhh)

## How to use?

DISCLAIMER: You need `rust-nightly` to carry out benchmarkings in Rust. The `test` crate
is still unstable. Once installed, run

```
cargo bench
```

for benchmarking and run

```
cargo test
```

to check that I haven't messed up the implementation of a sorting algorithm.
