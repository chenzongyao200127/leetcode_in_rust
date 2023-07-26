// 2569_Handling_Sum_Queries_After_Update
// https://leetcode.cn/problems/handling-sum-queries-after-update/

// 给你两个下标从 0 开始的数组 nums1 和 nums2 ，和一个二维数组 queries 表示一些操作。总共有 3 种类型的操作：

// 操作类型 1 为 queries[i] = [1, l, r] 。你需要将 nums1 从下标 l 到下标 r 的所有 0 反转成 1 或将 1 反转成 0 。l 和 r 下标都从 0 开始。
// 操作类型 2 为 queries[i] = [2, p, 0] 。对于 0 <= i < n 中的所有下标，令 nums2[i] = nums2[i] + nums1[i] * p 。
// 操作类型 3 为 queries[i] = [3, 0, 0] 。求 nums2 中所有元素的和。
// 请你返回一个数组，包含所有第三种操作类型的答案。

// 超时
impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut nums1 = nums1;
        let mut nums2: Vec<i64> = nums2.into_iter().map(|x| x as i64).collect();
        let mut ans = vec![];
        for query in queries {
            let t = query[0];
            let x = query[1] as usize;
            let y = query[2] as usize;
            match t {
                1 => {
                    for i in x..=y {
                        if nums1[i] == 0 || nums1[i] == 1 {
                            nums1[i] = (nums1[i] + 1) % 2;
                        }
                    }
                },
                2 => {
                    for i in 0..nums2.len() {
                        nums2[i] = nums2[i] as i64 + nums1[i] as i64 * x as i64;
                    }
                }
                _ => {
                    let tmp_sum = nums2.iter().sum::<i64>();
                    ans.push(tmp_sum);
                }
            }
        }
        ans
    }
}

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut nums1 = nums1;
        let mut nums2: Vec<i64> = nums2.into_iter().map(|x| x as i64).collect();
        let mut ans = vec![];

        for query in queries {
            let t = query[0];
            let x = query[1] as usize;
            let y = query[2] as usize;

            match t {
                1 => {
                    for i in x..=y {
                        if nums1[i] <= 1 {
                            nums1[i] ^= 1;
                        }
                    }
                },
                2 => {
                    for i in 0..nums2.len() {
                        nums2[i] += nums1[i] as i64 * x as i64;
                    }
                }
                _ => {
                    let tmp_sum = nums2.iter().sum::<i64>();
                    ans.push(tmp_sum);
                }
            }
        }

        ans
    }
}

// 没想到 numpy 能过
// import numpy as np
// class Solution:
//     def handleQuery(self, nums1: List[int], nums2: List[int], queries: List[List[int]]) -> List[int]:
//         m1, m2 = np.array(nums1), np.array(nums2)
//         res = []
//         for o, l, r in queries:
//             if o==1: m1[l: r+1] ^= 1
//             if o==2: m2 += m1*l
//             if o==3: res.append(int(m2.sum()))
//         return res

// Use the Lazy Segment Tree to process the queries quickly.