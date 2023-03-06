// fn main() {
//     let mut array = vec![47,2,71,99,78,19,24,47];
//     let len = array.len();
//     quick_sort(&mut array, 0, len -1 );
//     println!("{:?}", array);
// }




use rand::Rng;
use std::cmp::Ordering::*;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = n - k as usize;
        Self::quickselect(&mut nums, k)
    }

    fn quickselect<T: Ord + Copy>(arr: &mut [T], k: usize) -> T {
        let len = arr.len();
        let pivot = Self::partition(arr);
        match pivot.cmp(&k) {
            Equal => arr[pivot],
            Greater => Self::quickselect(&mut arr[0..pivot], k),
            Less => Self::quickselect(&mut arr[pivot + 1..len], k - pivot - 1),
        }
    }

    fn partition<T: Ord + Copy>(arr: &mut [T]) -> usize {
        let mut rng = rand::thread_rng();
        let len = arr.len();
        let pivot_index = rng.gen_range(0, len);
        arr.swap(pivot_index, len - 1);
        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }
}


pub fn quick_sort(array: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pi = array[left];
    let mut i = left;
    let mut j = right;
    while i < j {
        while i < j && array[j] > pi {
            j -= 1;
        }
        if i < j {
            array[i] = array[j];
            i += 1;
        }
        while i < j && array[i] < pi {
            i += 1;
        }
        if i < j {
            array[j] = array[i];
            j -= 1;
        }
    }
    array[i] = pi;
    quick_sort(array, left, i-1);
    quick_sort(array, i+1, right);
}