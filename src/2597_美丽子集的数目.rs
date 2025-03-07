// 2597_美丽子集的数目
// https://leetcode.cn/problems/the-number-of-beautiful-subsets/description/?envType=daily-question&envId=2025-03-07

// 给你一个由正整数组成的数组 nums 和一个 正整数 k 。
// 如果 nums 的子集中，任意两个整数的绝对差均不等于 k ，则认为该子数组是一个 美丽 子集。
// 返回数组 nums 中 非空 且 美丽 的子集数目。

// nums 的子集定义为：可以经由 nums 删除某些元素（也可能不删除）得到的一个数组。只有在删除元素时选择的索引不同的情况下，两个子集才会被视作是不同的子集。

// 1 <= nums.length <= 18
// 1 <= nums[i], k <= 1000

// Sort the array nums and create another array cnt of size nums[i].
// Use backtracking to generate all the beautiful subsets. If cnt[nums[i] - k] is positive, then it is impossible to add nums[i] in the subset, and we just move to the next index. Otherwise, it is also possible to add nums[i] in the subset, in this case, increase cnt[nums[i]], and move to the next index.

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(
            nums: &[i32],
            k: i32,
            cnt: &mut std::collections::HashMap<i32, i32>,
            index: usize,
            ans: &mut i32,
        ) {
            if index == nums.len() {
                *ans += 1;
                return;
            }
            dfs(nums, k, cnt, index + 1, ans);
            let x = nums[index];
            if *cnt.get(&(x - k)).unwrap_or(&0) == 0 && *cnt.get(&(x + k)).unwrap_or(&0) == 0 {
                *cnt.entry(x).or_insert(0) += 1; // 选
                dfs(nums, k, cnt, index + 1, ans); // 讨论 nums[i+1] 选或不选
                *cnt.entry(x).or_insert(0) -= 1; // 撤销，恢复现场
            }
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = std::collections::HashMap::new();
        let mut ans = -1; // 去掉空集
        dfs(&nums, k, &mut cnt, 0, &mut ans);
        ans
    }
}
