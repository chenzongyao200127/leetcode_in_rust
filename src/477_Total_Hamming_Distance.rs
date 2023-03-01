// 477. Total Hamming Distance
// https://leetcode.cn/problems/total-hamming-distance/

// 两个整数的 汉明距离 指的是这两个数字的二进制数对应位不同的数量。
// 给你一个整数数组 nums，请你计算并返回 nums 中任意两个数之间 汉明距离的总和 。
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                ans += (nums[i] ^ nums[j]).count_ones();
            }
        }

        ans as i32
    }
}

// 方法一：逐位统计
// 我们可以从二进制的最低位到最高位，逐位统计汉明距离。将每一位上得到的汉明距离累加即为答案。
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let (mut res, mut cnt, len) = (0, 0, nums.len() as i32);
        for i in 0..30 {
            for n in nums.iter() {
                cnt += (n >> i) & 1;
            }
            res += cnt * (len - cnt);
            cnt = 0;
        }
        res
    }
}