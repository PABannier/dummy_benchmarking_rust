#[inline]
pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    // O(n²) time
    for i in 0..(arr.len() - 1) {
        for j in i..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(i, j);
            }
        }
    }
}

pub mod merge_sort {
    #[inline]
    pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        // O(nLogn) time
        if arr.len() > 1 {
            let mid = arr.len() / 2;
            merge_sort(&mut arr[..mid]);
            merge_sort(&mut arr[mid..]);

            let mut out: Vec<T> = arr.to_vec();
            merge(&arr[..mid], &arr[mid..], &mut out[..]);
            arr.copy_from_slice(&out[..]);
        }
    }

    fn merge<T: PartialOrd + Copy>(arr1: &[T], arr2: &[T], out: &mut [T]) {
        let mut ptr1: usize = 0;
        let mut ptr2: usize = 0;
        let mut ptr3: usize = 0;

        while ptr1 < arr1.len() && ptr2 < arr2.len() {
            if arr1[ptr1] < arr2[ptr2] {
                out[ptr3] = arr1[ptr1];
                ptr1 += 1;
            } else {
                out[ptr3] = arr2[ptr2];
                ptr2 += 1;
            }
            ptr3 += 1;
        }

        while ptr1 < arr1.len() {
            out[ptr3] = arr1[ptr1];
            ptr1 += 1;
            ptr3 += 1;
        }

        while ptr2 < arr2.len() {
            out[ptr3] = arr2[ptr2];
            ptr2 += 1;
            ptr3 += 1;
        }
    }
}

pub mod quick_sort {
    #[inline]
    pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        // O(nLog(n)) time
        quick_sort_helper(arr, 0, arr.len() - 1);
    }

    fn quick_sort_helper<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize) {
        if start < end {
            let pivot = partition(arr, start, end);

            quick_sort_helper(arr, start, pivot - 1);
            quick_sort_helper(arr, pivot + 1, end);
        }
    }

    fn partition<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize) -> usize {
        let pivot = start;
        let mut left = start + 1;
        let mut right = end;

        while left <= right {
            if arr[left] > arr[pivot] && arr[right] < arr[pivot] {
                arr.swap(left, right);
            }
            if arr[left] <= arr[pivot] {
                left += 1;
            }
            if arr[right] >= arr[pivot] {
                right -= 1;
            }
        }
        arr.swap(pivot, right);
        right
    }
}

pub mod heap_sort {
    #[inline]
    pub fn heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let right = arr.len();
        let mid = right / 2;

        for start in (0..mid).rev() {
            sift_down(arr, start, right - 1);
        }

        for right in (1..arr.len()).rev() {
            arr.swap(right, 0);
            sift_down(arr, 0, right - 1);
        }
    }

    fn sift_down<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) {
        let mut root = left;
        loop {
            let mut child = root * 2 + 1;
            if child > right {
                break;
            }
            if child < right && arr[child] < arr[child + 1] {
                child += 1;
            }

            if arr[root] < arr[child] {
                arr.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }
}

pub mod radix_sort {
    use std::ops::Div;
    use std::ops::DivAssign;

    #[inline]
    pub fn radix_sort<T>(arr: &mut [T])
    where
        T: PartialOrd + From<u64> + Copy + DivAssign + Div<Output = u64>,
    {
        if arr.len() == 0 {
            return;
        }

        let mut max_num: T = T::from(0);
        for idx in 0..arr.len() {
            if arr[idx] > max_num {
                max_num = arr[idx];
            }
        }

        let mut digit_column = 1;
        loop {
            if max_num / T::from(digit_column) <= 0 {
                break;
            }
            counting_sort(arr, digit_column);
            digit_column *= 10;
        }
    }

    fn counting_sort<T>(arr: &mut [T], digit_column: u64)
    where
        T: PartialOrd + From<u64> + Copy + DivAssign + Div<Output = u64>,
    {
        let mut counts = [0i32; 10];
        let mut sorted: Vec<T> = vec![T::from(0); arr.len()];

        for idx in 0..arr.len() {
            let count_idx = (arr[idx] / T::from(digit_column)) % 10;
            counts[count_idx as usize] += 1;
        }

        for idx in 1..counts.len() {
            counts[idx] += counts[idx - 1];
        }

        let mut idx = arr.len() - 1;
        loop {
            let count_idx = (arr[idx] / T::from(digit_column)) % 10;
            counts[count_idx as usize] -= 1;
            sorted[counts[count_idx as usize] as usize] = arr[idx];
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        arr.copy_from_slice(&sorted[..]);
    }
}
