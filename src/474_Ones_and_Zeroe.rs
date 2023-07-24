// 474_Ones_and_Zeroes
// https://leetcode.cn/problems/ones-and-zeroes/description/

// 给你一个二进制字符串数组 strs 和两个整数 m 和 n 。
// 请你找出并返回 strs 的最大子集的长度，该子集中 最多 有 m 个 0 和 n 个 1 。
// 如果 x 的所有元素也是 y 的元素，集合 x 是集合 y 的 子集 。

use::std::cmp::max;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut new_strs = vec![];
        for s in strs {
            new_strs.push(str2vec(s));
        }

        let len = new_strs.len();
        let mut dp = vec![vec![vec![0; n as usize + 1]; m as usize + 1]; len + 1];
        for i in 0..=m as usize {
            for j in 0..=n as usize {
                dp[0][i][j] = 0;
            }
        }

        for i in 1..=len {
            for x in 0..=m as usize {
                for y in 0..=n as usize {
                    let zeros = new_strs[i-1][0];
                    let ones = new_strs[i-1][1];
                    dp[i][x][y] = dp[i-1][x][y];
                    if x >= zeros && y >= ones {
                        dp[i][x][y] = max(dp[i][x][y], dp[i-1][x-zeros][y-ones] + 1);
                    }
                }
            }
        }

        dp[len][m as usize][n as usize]
    }

}

pub fn str2vec(s: String) -> Vec<usize> {
    let mut ans = vec![0; 2];
    s.chars().into_iter().for_each(|c| {
        if c == '0' {
            ans[0] += 1;
        } else {
            ans[1] += 1;
        }
    });

    ans
}




// 评论区
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        fn count(s: &str) -> (usize, usize) {
            let m = s
                .chars()
                .fold(0, |acc, c| acc + if c == '0' { 1 } else { 0 });
            (m, s.len() - m)
        }

        let mut dp = vec![vec![0; m as usize + 1]; n as usize + 1];
        strs.iter().for_each(|s| {
            let (ms, ns) = count(s);
            for ni in (ns..=n as usize).rev() {
                for mi in (ms..=m as usize).rev() {
                    dp[ni][mi] = dp[ni][mi].max(dp[ni - ns][mi - ms] + 1);
                }
            }
        });

        dp[n as usize][m as usize]
    }
}
