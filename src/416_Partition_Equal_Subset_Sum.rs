// 416_Partition_Equal_Subset_Sum
// https://leetcode.cn/problems/partition-equal-subset-sum/

// 给你一个 只包含正整数 的 非空 数组 nums 。
// 请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。

// 错误解答
// 没有逆序遍历
// 在这个问题中，我们需要逆序遍历是因为我们需要避免在一次迭代中重复计算。
// 也就是说，当我们考虑是否选择当前的数字`num`时，我们需要确保我们计算`dp[i]`时用到的`dp[i-num]`是在添加当前数字`num`之前计算得到的。

// 如果我们正序遍历，当我们在计算`dp[i]`时，`dp[i-num]`可能已经在这一轮迭代中被更新过了，那么`dp[i]`实际上就是考虑了选择`num`两次的情况。

// 让我们举个例子来说明一下。比如说我们有一个数组`[1,2,5]`，
// 我们想要查看能否找到一个子集和为4。如果我们在处理数字2时正序遍历，
// 那么在我们计算`dp[4]`时，`dp[2]`可能已经在这一轮迭代中被更新过了（因为`dp[2] = dp[2] || dp[2-2]`），
// 也就是说`dp[4]`在这种情况下实际上考虑了选择数字2两次的情况，这显然是我们不希望发生的。

// 因此，为了确保我们在处理`dp[i]`时，`dp[i-num]`是在选择`num`之前计算得到的，我们需要逆序遍历。
// 这样我们就可以保证我们在计算`dp[i]`时，`dp[i-num]`是在选择`num`之前计算得到的，也就是说，`dp[i]`是基于选择`num`一次的情况计算得到的。
// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let total_sum = nums.iter().sum::<i32>();
//         if total_sum % 2 == 1 {
//             return false;
//         }

//         let target = total_sum / 2;
//         let mut dp = vec![false; target as usize + 1];
//         let mut nums = nums;
//         dp[0] = true;
//         nums.sort_unstable();
//         for num in nums {
//             for i in num as usize..=target as usize {
//                 dp[i] = dp[i] || dp[i-num as usize];
//             }
//         }

//         return dp[target as usize]
//     }
// }

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 == 1 {
            return false;
        }

        let target = total_sum / 2;
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;
        for &num in nums.iter() {
            for i in (num as usize..=target as usize).rev() {
                dp[i] = dp[i] || dp[i - num as usize];
            }
        }

        dp[target as usize]
    }
}


// 优化
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 {
            return false;
        }
        let target = sum >> 1;
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for num in nums {
            // 符合条件的话就提取返回
            if target >= num && (dp[target as usize] || dp[(target - num) as usize]) {
                return true;
            }
            for j in (num..target).rev() {
                dp[j as usize] = dp[j as usize] || dp[(j - num) as usize];
            }
        }
        dp[target as usize]
    }
}
