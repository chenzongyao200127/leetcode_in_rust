# 2771_Longest_Non-decreasing_Subarray_From_Two_Arrays
# https://leetcode.cn/problems/longest-non-decreasing-subarray-from-two-arrays/description/

# 给你两个下标从 0 开始的整数数组 nums1 和 nums2 ，长度均为 n 。

# 让我们定义另一个下标从 0 开始、长度为 n 的整数数组，nums3 。
# 对于范围 [0, n - 1] 的每个下标 i ，你可以将 nums1[i] 或 nums2[i] 的值赋给 nums3[i] 。

# 你的任务是使用最优策略为 nums3 赋值，以最大化 nums3 中 最长非递减子数组 的长度。
# 以整数形式表示并返回 nums3 中 最长非递减 子数组的长度。
# 注意：子数组 是数组中的一个连续非空元素序列。

# 示例 1：
# 输入：nums1 = [2,3,1], nums2 = [1,2,1]
# 输出：2
# 解释：构造 nums3 的方法之一是： 
# nums3 = [nums1[0], nums2[1], nums2[2]] => [2,2,1]
# 从下标 0 开始到下标 1 结束，形成了一个长度为 2 的非递减子数组 [2,2] 。 
# 可以证明 2 是可达到的最大长度。

# 示例 2：
# 输入：nums1 = [1,3,2,1], nums2 = [2,2,3,4]
# 输出：4
# 解释：构造 nums3 的方法之一是： 
# nums3 = [nums1[0], nums2[1], nums2[2], nums2[3]] => [1,2,3,4]
# 整个数组形成了一个长度为 4 的非递减子数组，并且是可达到的最大长度。

# 示例 3：
# 输入：nums1 = [1,1], nums2 = [2,2]
# 输出：2
# 解释：构造 nums3 的方法之一是： 
# nums3 = [nums1[0], nums1[1]] => [1,1] 
# 整个数组形成了一个长度为 2 的非递减子数组，并且是可达到的最大长度。


# 题解
# 为方便后面翻译成递推，这里从右往左递归。
# 定义 dfs(i,j) 表示以 nums_j[i] 结尾的最长非递减子数组的长度


class Solution:
    def maxNonDecreasingLength(self, nums1: List[int], nums2: List[int]) -> int: