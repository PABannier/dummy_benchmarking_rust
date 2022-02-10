#![feature(test)]
extern crate test;

mod sort;

#[cfg(test)]
mod tests {
    use super::sort::{bubble_sort, merge_sort, heap_sort};
    use test::Bencher;
    use rand::Rng;


    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let mut arr: Vec<u64> = (0..100).map(|_| rng.gen_range(0..1000)).collect();
        let mut arr2 = arr.to_owned();
        let mut arr3 = arr.to_owned();
    
        bubble_sort(&mut arr);
        merge_sort(&mut arr2);
        heap_sort(&mut arr3);
    
        assert_eq!(arr, arr2);
        assert_eq!(arr2, arr3);
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut arr: Vec<u64> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
            bubble_sort(&mut arr);
        });
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut arr: Vec<u64> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
            merge_sort(&mut arr);
        })
    }

    #[bench]
    fn bench_heap_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let mut arr: Vec<u64> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();
            heap_sort(&mut arr);
        })
    }
}


fn main() {
   
}