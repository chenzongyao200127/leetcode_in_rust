// 1186_Maximum_Subarray_Sum_with_One_Deletion
// https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/


// 给你一个整数数组，返回它的某个 非空 子数组（连续元素）在执行一次可选的删除操作后，所能得到的最大元素总和。
// 换句话说，你可以从原数组中选出一个子数组，并可以决定要不要从中删除一个元素（只能删一次哦），
// （删除后）子数组中至少应当有一个元素，然后该子数组（剩下）的元素总和是所有子数组之中最大的。

// 注意，删除一个元素后，子数组 不能为空。

// How to solve this problem if no deletions are allowed ?

// Try deleting each element and find the maximum subarray sum to both sides of that element.

// To do that efficiently, use the idea of Kadane's algorithm.

// 为了解决这个问题，我们可以使用动态规划。我们需要维护两个状态数组，dp_keep 和 dp_delete。
// dp_keep[i] 表示以第 i 个元素为结尾的子数组的最大和，不删除任何元素；
// dp_delete[i] 表示以第 i 个元素为结尾的子数组的最大和，可以删除一个元素。

// 具体算法如下：
// 初始化 dp_keep[0] 和 dp_delete[0]，它们都等于 arr[0]。
// 遍历数组中的其他元素（从索引 1 开始）。
// 对于每个元素：
// 更新 dp_keep[i]，其值为 max(dp_keep[i - 1] + arr[i], arr[i])。
// 更新 dp_delete[i]，其值为 max(dp_keep[i - 1], dp_delete[i - 1] + arr[i], arr[i])。
// 遍历结束后，找到 dp_delete 数组中的最大值，即为解。
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut dp_keep = vec![i32::MIN; len];
        let mut dp_delete = vec![i32::MIN; len];
        dp_keep[0] = arr[0];
        dp_delete[0] = arr[0];
        for i in 1..len {
            dp_keep[i] = arr[i].max(dp_keep[i - 1] + arr[i]);
            dp_delete[i] = arr[i].max(dp_keep[i - 1].max(dp_delete[i - 1] + arr[i]));
        }
        return dp_delete.into_iter().max().unwrap()
    }
}


// Kadane的算法（Kadane's Algorithm）是一种用于解决最大子数组和问题的有效方法。最大子数组和问题是在一个给定的整数数组中找到连续子数组，使得子数组的和最大。Kadane的算法使用动态规划来求解，它的时间复杂度为O(n)，其中n是数组的长度。

// 这是Kadane算法的步骤：

// 初始化两个变量，一个是max_so_far用于存储目前为止找到的最大子数组和，另一个是max_ending_here用于存储以当前元素结尾的最大子数组和。将两者都初始化为数组的第一个元素的值。

// 遍历数组的其余元素（从索引1开始）。

// 对于每个元素，计算以该元素结尾的最大子数组和。这可以通过比较max_ending_here + current_element和current_element来实现。如果max_ending_here + current_element比current_element大，那么更新max_ending_here为max_ending_here + current_element；否则，将max_ending_here设置为current_element。

// 接下来，更新max_so_far。如果max_ending_here比max_so_far大，更新max_so_far为max_ending_here的值。

// 遍历完数组后，max_so_far就是最大子数组和。