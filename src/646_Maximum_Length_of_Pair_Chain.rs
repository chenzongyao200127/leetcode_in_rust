// 646. Maximum Length of Pair Chain
// https://leetcode.cn/problems/maximum-length-of-pair-chain/


// 646. 最长数对链
// 给你一个由 n 个数对组成的数对数组 pairs ，其中 pairs[i] = [lefti, righti] 且 lefti < righti 。
// 现在，我们定义一种 跟随 关系，当且仅当 b < c 时，数对 p2 = [c, d] 才可以跟在 p1 = [a, b] 后面。我们用这种形式来构造 数对链 。
// 找出并返回能够形成的 最长数对链的长度 。
// 你不需要用到所有的数对，你可以以任何顺序选择其中的一些数对来构造。
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_unstable_by(|x, y| x[1].cmp(&y[1]));
        let mut curr = i32::MIN;
        let mut ans = 0;
        for pair in pairs {
            if curr < pair[0] {
                curr = pair[1];
                ans += 1;
            }
        }
        ans
    }
}    


impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.len();
        let mut pairs = pairs;
        pairs.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if pair[i][0] > pairs[j][1] {
                    dp[i] = dp[1].max(dp[j] + 1);
                }
            }
        }
        dp[n-1]
    }
}

