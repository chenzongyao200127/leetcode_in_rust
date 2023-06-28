// 1681_Minimum_Incompatibility
// https://leetcode.cn/problems/minimum-incompatibility/


// 这个实现首先对原始数组进行排序，然后使用回溯法递归地将数字添加到子集中。
// 为了避免重复数字，我们需要检查当前子集是否已经包含了相同的数字。
// 通过剪枝，我们可以在不兼容性超过当前最小值时停止搜索。
// 最后，返回最小的不兼容性之和。如果找不到有效解，返回 -1。
// 超时 难绷
impl Solution {
    pub fn minimum_incompatibility(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let group_size = n / k as usize;
        if n % k as usize != 0 {
            return -1;
        }
    
        nums.sort_unstable();
        let mut groups = vec![vec![]; k as usize];
        let mut min_incompatibility = i32::MAX;
    
        fn backtrack(
            nums: &[i32],
            groups: &mut Vec<Vec<i32>>,
            k: usize,
            group_size: usize,
            incompatibility: i32,
            min_incompatibility: &mut i32,
        ) {
            if nums.is_empty() {
                *min_incompatibility = i32::min(*min_incompatibility, incompatibility);
                return;
            }
            
            if incompatibility >= *min_incompatibility {
                return;
            }
    
            let num = nums[0];
            let remaining_nums = &nums[1..];
            for i in 0..k {
                if groups[i].len() < group_size && !groups[i].contains(&num) {
                    let prev_incompatibility = if groups[i].is_empty() {
                        0
                    } else {
                        num - groups[i][0]
                    };
                    groups[i].push(num);
                    backtrack(
                        remaining_nums,
                        groups,
                        k,
                        group_size,
                        incompatibility + prev_incompatibility,
                        min_incompatibility,
                    );
                    groups[i].pop();
                }
            }
        }
    
        backtrack(&nums, &mut groups, k as usize, group_size, 0, &mut min_incompatibility);
        if min_incompatibility == i32::MAX {
            -1
        } else {
            min_incompatibility
        }
    }
}

use std::collections::HashMap;
use std::cmp::min;


impl Solution {
    pub fn minimum_incompatibility(a: Vec<i32>, k: i32) -> i32 {
        let counter = a.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

        if counter.values().any(|&c| c > k) {
            return -1;
        }

        let n = a.len();
        let size = n / k as usize;
        let mut a = a.clone();
        a.sort();

        let mut memo = HashMap::new();

        fn dfs(a: &[i32], left: i32, pre: usize, size: usize, memo: &mut HashMap<(i32, usize), i32>) -> i32 {
            if left == 0 {
                return 0;
            }
            if left.count_ones() as usize % size == 0 {
                let lb = left & -left;
                return dfs(a, left ^ lb, lb.trailing_zeros() as usize - 1, size, memo);
            }

            if let Some(&cached) = memo.get(&(left, pre)) {
                return cached;
            }

            let mut res = i32::MAX;
            let mut last = a[pre];
            let n = a.len();
            for i in (pre + 1)..n {
                if (left >> i) & 1 == 1 && a[i] != last {
                    last = a[i];
                    res = min(res, last - a[pre] + dfs(a, left ^ (1 << i), i, size, memo));
                }
            }

            memo.insert((left, pre), res);
            res
        }

        dfs(&a, (1 << n) - 2, 0, size, &mut memo)
    }
}

// class Solution:
//     def minimumIncompatibility(self, a: List[int], k: int) -> int:
//         # 首先，通过鸽巢原理检查是否有重复元素的个数大于 k。
//         # 如果存在这种情况，说明无法将数组分为 k 个子集，因此返回 -1。
//         if any(c > k for c in Counter(a).values()):  # 鸽巢原理
//             return -1
		
//         # 计算子集的大小，并对数组进行排序，以便后续判断重复元素。
//         n = len(a)
//         size = n // k
//         a.sort()  # 排序，便于判断重复

//         # 定义一个带有缓存的深度优先搜索函数。
//         # left 表示待处理的数字的二进制表示，pre 表示上一个处理的数字在数组 a 中的索引。
//         @cache
//         def dfs(left: int, pre: int) -> int:
//             # 如果没有待处理的数字，直接返回 0。
//             if left == 0: return 0
        
//         	   # 如果剩余待处理的数字个数可以整除子集大小，说明需要创建一个新的子集。
//             # 这时我们选择最小的待处理数字作为新子集的第一个数字，并递归调用 dfs 函数。
//             if left.bit_count() % size == 0:  # 创建一个新的组
//                 lb = left & -left  # 选择 lowbit 作为第一个数
//                 return dfs(left ^ lb, lb.bit_length() - 1)
//             # 初始化子集不兼容性结果为无穷大。然后遍历数组 a，在当前子集中找到下一个数字。
//             res = inf
//             last = a[pre]
            
//             # 这里检查待处理数字中是否包含当前数字并且是否不重复。
//             # 如果满足条件，我们计算子集的不兼容性，并更新结果。
//             for i in range(pre + 1, n):  # 枚举这个组的下一个数
//                 if left >> i & 1 and a[i] != last:  # 组内不能有重复数字，且 a 中重复数字只需枚举一次
//                     last = a[i]
//                     res = min(res, last - a[pre] + dfs(left ^ (1 << i), i))
//             return res
		
//         # 最后，从第一个数字开始调用 dfs 函数。
//         return dfs((1 << n) - 2, 0)