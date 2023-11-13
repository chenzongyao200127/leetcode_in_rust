// 100117_Minimum_Operations_to_Maximize_Last_Elements_in_Arrays
// https://leetcode.cn/problems/minimum-operations-to-maximize-last-elements-in-arrays/


// 给你两个下标从 0 开始的整数数组 nums1 和 nums2 ，这两个数组的长度都是 n 。

// 你可以执行一系列 操作（可能不执行）。

// 在每次操作中，你可以选择一个在范围 [0, n - 1] 内的下标 i ，并交换 nums1[i] 和 nums2[i] 的值。

// 你的任务是找到满足以下条件所需的 最小 操作次数：

// nums1[n - 1] 等于 nums1 中所有元素的 最大值 ，即 nums1[n - 1] = max(nums1[0], nums1[1], ..., nums1[n - 1]) 。
// nums2[n - 1] 等于 nums2 中所有元素的 最大值 ，即 nums2[n - 1] = max(nums2[0], nums2[1], ..., nums2[n - 1]) 。

// 以整数形式，表示并返回满足上述 全部 条件所需的 最小 操作次数，如果无法同时满足两个条件，则返回 -1 。

impl Solution {
    pub fn min_operations(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let max1 = nums1.iter().max().unwrap();
        let max2 = nums2.iter().max().unwrap();

        if nums1[nums1.len() - 1] == *max1 && nums2[nums2.len() - 1] == *max2 {
            return 0;
        }

        #[inline]
        fn helper(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
            let mut ans = 0;

            let last1 = nums1.iter().last().unwrap();
            let last2 = nums2.iter().last().unwrap();
            
            for i in 0..nums1.len()-1 {
                if nums1[i] <= *last1 && nums2[i] <= *last2 {
                    continue;
                } else if nums1[i] <= *last2 && nums2[i] <= *last1 {
                    ans += 1
                } else {
                    ans = nums1.len() as i32;
                    break;
                }
            }

            ans
        }

        let x = helper(&nums1, &nums2);

        // swap
        let tmp = nums1[nums1.len() - 1];
        let n = nums1.len();
        nums1[n - 1] = nums2[n - 1];
        nums2[n - 1] = tmp;

        let y = helper(&nums1, &nums2) + 1;

        let ans = x.min(y);
        
        if ans == n as i32 {
            return -1
        } else {
            return ans
        }
    }
}