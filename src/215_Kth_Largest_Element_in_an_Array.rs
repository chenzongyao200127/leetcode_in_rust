// 215. Kth Largest Element in an Array
// https://leetcode.cn/problems/kth-largest-element-in-an-array/

// 给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

// 请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

// 你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let rev_k = nums.len() - k as usize;
        let mut nums = nums;
        let len = nums.len()-1;
        quick_k_sort(&mut nums, 0, len, rev_k);
        nums[rev_k]
    }
}


pub fn quick_k_sort(nums: &mut Vec<i32>, left: usize, right: usize, rev_k: usize) {
    if left >= right {
        return;
    }

    let pi = nums[left];
    let mut i = left;
    let mut j = right;
    while i < j {
        while i < j && nums[j] > pi {
            j -= 1;
        }
        if i < j {
            nums[i] = nums[j];
            i += 1;
        }
        while i < j && nums[i] < pi {
            i += 1;
        }
        if i < j {
            nums[j] = nums[i];
            j -= 1;
        }
    }
    nums[i] = pi;

    if rev_k == i {
        return;
    } else if rev_k < i {
        quick_k_sort(nums, left, i-1, rev_k);
    } else {
        quick_k_sort(nums, i+1, right, rev_k);
    }
}

// CHEAT
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len()-k as usize]
    }
}

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