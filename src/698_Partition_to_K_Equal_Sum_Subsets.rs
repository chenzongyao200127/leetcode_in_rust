// 698_Partition_to_K_Equal_Sum_Subsets
// https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/description/

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        if total % k != 0 {
            return false;
        }

        let target = total / k;
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        
        if nums[0] > target {
            return false;
        }

        let mut visited = vec![false; nums.len()];
        Self::can_partition(&nums, &mut visited, 0, k, 0, target)
    }

    fn can_partition(nums: &Vec<i32>, visited: &mut Vec<bool>, start: usize, k: i32, curr_sum: i32, target: i32) -> bool {
        if k == 1 {
            return true;
        }

        if curr_sum == target {
            return  Self::can_partition(nums, visited, 0, k - 1, 0, target);
        }
        
        for i in start..nums.len() {
            if !visited[i] && curr_sum + nums[i] <= target {
                visited[i] = true;
                if  Self::can_partition(nums, visited, i + 1, k, curr_sum + nums[i], target) {
                    return true;
                }
                visited[i] = false;
            }
        }
        false
    }
}