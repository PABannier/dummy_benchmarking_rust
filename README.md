A dummy repo to learn how to effectively benchmark in Rust. As usual,
sorting algorithms are of great help to set things up and quicky prototype.

A quick results on benchmarking for a vector of 1000 integers to sort:

- Bubble sort: `1.2 x 10⁶ ns` `O(n²)` (Arghhh...)
- Merge sort: `72 x 10³ ns` `O(nLogn)` (Youpi!)
- Heap sort: `58 x 10³ ns` `O(nLogn)` (Waouhhhhh)