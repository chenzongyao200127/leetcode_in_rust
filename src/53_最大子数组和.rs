// 53_最大子数组和

// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
// 子数组是数组中的一个连续部分。
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Helper function to calculate the maximum subarray sum using divide and conquer
        fn helper(nums: &[i32]) -> (i32, i32, i32, i32) {
            // Base case: if the array has only one element
            if nums.len() == 1 {
                let val = nums[0];
                return (val, val, val, val); // (left sum, right sum, max sum, total sum)
            }

            // Divide the array into two halves
            let mid = nums.len() / 2;
            let (left_lsum, left_rsum, left_msum, left_tsum) = helper(&nums[..mid]); // Left half
            let (right_lsum, right_rsum, right_msum, right_tsum) = helper(&nums[mid..]); // Right half

            // Calculate the left sum, right sum, max sum, and total sum for the current array
            let lsum = left_lsum.max(left_tsum + right_lsum); // Max sum starting from the left
            let rsum = right_rsum.max(right_tsum + left_rsum); // Max sum ending at the right
            let msum = left_msum.max(right_msum).max(left_rsum + right_lsum); // Max subarray sum
            let tsum = left_tsum + right_tsum; // Total sum of the array

            (lsum, rsum, msum, tsum)
        }

        // Call the helper function and return the maximum subarray sum
        let (_, _, msum, _) = helper(&nums);
        msum
    }
}
