#![feature(test)]
extern crate test;

mod sort;

#[cfg(test)]
mod tests {
    use super::sort::{
        bubble_sort, heap_sort::heap_sort, merge_sort::merge_sort, quick_sort::quick_sort,
    };
    use rand::prelude::*;
    use rand_pcg::Pcg64;
    use rand_seeder::Seeder;
    use test::Bencher;

    fn get_unsorted_vector(random_state: usize) -> Vec<u64> {
        let mut rng: Pcg64 = Seeder::from(random_state).make_rng();
        let arr: Vec<u64> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
        arr
    }

    #[test]
    fn it_works() {
        let mut arr = get_unsorted_vector(0);
        let mut arr2 = arr.to_owned();
        let mut arr3 = arr.to_owned();
        let mut arr4 = arr.to_owned();

        bubble_sort(&mut arr);
        merge_sort(&mut arr2);
        heap_sort(&mut arr3);
        quick_sort(&mut arr4);

        assert_eq!(arr, arr2);
        assert_eq!(arr2, arr3);
        assert_eq!(arr3, arr4);
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        let mut arr = get_unsorted_vector(0);
        b.iter(|| {
            bubble_sort(&mut arr);
        });
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        let mut arr = get_unsorted_vector(0);
        b.iter(|| {
            merge_sort(&mut arr);
        })
    }

    #[bench]
    fn bench_heap_sort(b: &mut Bencher) {
        let mut arr = get_unsorted_vector(0);
        b.iter(|| {
            heap_sort(&mut arr);
        })
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        let mut arr = get_unsorted_vector(0);
        b.iter(|| {
            quick_sort(&mut arr);
        })
    }
}

fn main() {}
