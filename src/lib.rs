#![feature(test)]
extern crate test;

pub mod sort;

use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub fn get_unsorted_vector(random_state: usize, vec_size: u64) -> Vec<u64> {
    let mut rng: Pcg64 = Seeder::from(random_state).make_rng();
    let arr: Vec<u64> = (0..vec_size).map(|_| rng.gen_range(0..vec_size)).collect();
    arr
}

#[cfg(test)]
mod tests {
    use super::get_unsorted_vector;
    use super::sort::{
        bubble_sort, heap_sort::heap_sort, merge_sort::merge_sort, quick_sort::quick_sort,
        radix_sort::radix_sort,
    };

    #[test]
    fn test_bubble_sort() {
        let mut arr = get_unsorted_vector(0);
        bubble_sort(&mut arr);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = get_unsorted_vector(1);
        merge_sort(&mut arr);
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = get_unsorted_vector(2);
        heap_sort(&mut arr);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = get_unsorted_vector(3);
        quick_sort(&mut arr);
    }

    #[test]
    fn test_radix_sort() {
        let mut arr = get_unsorted_vector(4);
        radix_sort(&mut arr);
    }
}
