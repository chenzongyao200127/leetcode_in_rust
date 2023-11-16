// https://leetcode.cn/problems/find-k-closest-elements/

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut indexed_diff: Vec<_> = arr.iter()
                                          .enumerate()
                                          .map(|(idx, &n)| (idx, (n - x).abs()))
                                          .collect();
    
        // Partially sort the array to find the kth element
        let kth = k as usize - 1;
        indexed_diff.select_nth_unstable_by(kth, |a, b| {
            a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
        });
    
        // Extract the first k elements and sort them
        let mut result: Vec<i32> = indexed_diff[..=kth].iter().map(|&(idx, _)| arr[idx]).collect();
        result.sort_unstable();
    
        result
    }
}


impl Solution {
    fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut arr = arr.clone();
        
        // Sorting the array based on the absolute difference from x
        arr.sort_by_key(|&v| (v - x).abs());

        // Getting the first k elements
        let mut closest = arr.into_iter().take(k as usize).collect::<Vec<i32>>();

        // Sorting the resulting vector in non-decreasing order
        closest.sort_unstable();

        closest
    }
}


impl Solution {
    fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = arr.len();
        let mut left = 0;
        let mut right = n - k as usize;

        while left < right {
            let mid = left + (right - left) / 2;
            if x - arr[mid] <= arr[mid + k as usize] - x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        arr[left..left + k as usize].to_vec()
    }
}
