// 350. Intersection of Two Arrays II
// https://leetcode.cn/problems/intersection-of-two-arrays-ii/

// 350. 两个数组的交集 II
// 给你两个整数数组 nums1 和 nums2 ，请你以数组形式返回两数组的交集。
// 返回结果中每个元素出现的次数，应与元素在两个数组中都出现的次数一致（如果出现次数不一致，则考虑取较小值）。可以不考虑输出结果的顺序。

// 进阶：
// 如果给定的数组已经排好序呢？你将如何优化你的算法？
// 如果 nums1 的大小比 nums2 小，哪种方法更优？
// 如果 nums2 的元素存储在磁盘上，内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？

// 由于同一个数字在两个数组中都可能出现多次，因此需要用哈希表存储每个数字出现的次数。对于一个数字，其在交集中出现的次数等于该数字在两个数组中出现次数的最小值。

// 首先遍历第一个数组，并在哈希表中记录第一个数组中的每个数字以及对应出现的次数，然后遍历第二个数组，
// 对于第二个数组中的每个数字，如果在哈希表中存在这个数字，则将该数字添加到答案，并减少哈希表中该数字出现的次数。

// 为了降低空间复杂度，首先遍历较短的数组并在哈希表中记录每个数字以及对应出现的次数，然后遍历较长的数组得到交集。


impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();
        let (mut p1, mut p2) = (0,0);
        while p1 < nums1.len() && p2 < nums2.len() {
            if nums1[p1 as usize] == nums2[p2 as usize] {
                ans.push(nums1[p1 as usize]);
                p1 += 1;
                p2 += 1;
            } else if nums1[p1 as usize] < nums2[p2 as usize] {
                p1 += 1;
            } else {
                p2 += 1;
            }
        }

        ans
    }
}