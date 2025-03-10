// 2234_花园的最大总美丽值
// https://leetcode.cn/problems/maximum-total-beauty-of-the-gardens/description/?envType=daily-question&envId=2025-03-08

// Alice 是 n 个花园的园丁，她想通过种花，最大化她所有花园的总美丽值。

// 给你一个下标从 0 开始大小为 n 的整数数组 flowers ，其中 flowers[i] 是第 i 个花园里已经种的花的数目。已经种了的花 不能 移走。同时给你 newFlowers ，表示 Alice 额外可以种花的 最大数目 。同时给你的还有整数 target ，full 和 partial 。

// 如果一个花园有 至少 target 朵花，那么这个花园称为 完善的 ，花园的 总美丽值 为以下分数之 和 ：
// * 完善 花园数目乘以 full.
// * 剩余 不完善 花园里，花的 最少数目 乘以 partial 。如果没有不完善花园，那么这一部分的值为 0 。
// 请你返回 Alice 种最多 newFlowers 朵花以后，能得到的 最大 总美丽值。
// 注意，Alice可以让所有花园都变成完善的，但这样她的总美丽值反而更小。

use std::cmp::{max, min};

impl Solution {
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let mut flowers = flowers.clone();
        let n = flowers.len();
        for flower in flowers.iter_mut() {
            if *flower > target {
                *flower = target;
            }
        }
        flowers.sort_by(|a, b| b.cmp(a));
        let mut sum: i64 = flowers.iter().map(|&flower| flower as i64).sum();
        let mut ans = 0;
        if (target as i64) * n as i64 - sum <= new_flowers {
            ans = full as i64 * n as i64;
        }
        let mut pre = 0;
        let mut ptr = 0;
        for i in 0..n {
            if i != 0 {
                pre += flowers[i - 1] as i64;
            }
            if flowers[i] == target {
                continue;
            }
            let mut rest = new_flowers - ((target as i64) * i as i64 - pre);
            if rest < 0 {
                break;
            }
            while !(ptr >= i && (flowers[ptr] as i64) * (n - ptr) as i64 - sum <= rest) {
                sum -= flowers[ptr] as i64;
                ptr += 1;
            }
            rest -= (flowers[ptr] as i64) * (n - ptr) as i64 - sum;
            ans = max(
                ans,
                full as i64 * i as i64
                    + partial as i64
                        * min(
                            flowers[ptr] as i64 + rest / (n - ptr) as i64,
                            (target - 1) as i64,
                        ),
            );
        }
        ans
    }
}
