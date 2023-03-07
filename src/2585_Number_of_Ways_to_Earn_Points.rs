// 2585. Number of Ways to Earn Points
// https://leetcode.cn/problems/number-of-ways-to-earn-points/

// 2585. 获得分数的方法数
// 考试中有 n 种类型的题目。给你一个整数 target 和一个下标从 0 开始的二维整数数组 types ，其中 types[i] = [counti, marksi] 表示第 i 种类型的题目有 counti 道，每道题目对应 marksi 分。
// 返回你在考试中恰好得到 target 分的方法数。由于答案可能很大，结果需要对 109 +7 取余。
// 注意，同类型题目无法区分。
// 比如说，如果有 3 道同类型题目，那么解答第 1 和第 2 道题目与解答第 1 和第 3 道题目或者第 2 和第 3 道题目是相同的。

// 提示：
// 1 <= target <= 1000
// n == types.length
// 1 <= n <= 50
// types[i].length == 2
// 1 <= counti, marksi <= 50


impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let _MOD = 1000_000_007;
        let type_len = types.len();

        let mut dp = vec![vec![0; target as usize + 1]; type_len + 1];
        dp[0][0] = 1; //dp[i][j] 前i题获得j分数所有的方法数
        for i in 1..=type_len {
            let tmp_score = types[i-1][1];
            for j in 0..=target as usize {
                let mut k = 0;
                while k <= types[i-1][0] && (j as i32 - tmp_score * k >= 0) {
                    dp[i][j] += dp[i-1][j-(k*tmp_score) as usize];
                    dp[i][j] = dp[i][j] % _MOD;
                    k += 1;
                }
            }
        }

        dp[type_len][target as usize]
    }
}

///解法：多重背包
// 多重背包模板题，多重背包的讲解见 OI Wiki。https://leetcode.cn/link/?target=https://oi-wiki.org/dp/knapsack/#%E5%A4%9A%E9%87%8D%E8%83%8C%E5%8C%85
// 本题由于数据范围较小，直接使用复杂度为 target * sum(count) 的做法即可通过。
// class Solution {
//     public:
//         int waysToReachTarget(int target, vector<vector<int>>& types) {
//             int n = types.size();
//             const int MOD = 1e9 + 7;

//             long long f[target + 1];
//             memset(f, 0, sizeof(f));
//             f[0] = 1;
//             for (auto &vec : types) for (int i = target; i >= 0; i--) for (int j = 1; j <= vec[0]; j++)
//                 if (i >= j * vec[1]) f[i] = (f[i] + f[i - j * vec[1]]) % MOD;
//             return f[target];
//         }
//     };



// const int MOD = 1e9 + 7;

// class Solution {
// public:
//     int waysToReachTarget(int t, vector<vector<int>>& types) {
//         int n = types.size();
//         long long f[t + 1];

//         memset(f, 0, sizeof f);
//         f[0] = 1;
//         for (int i = 1; i <= n; i ++)
//             for (int j = t; j >= 0; j --)
//                 for (int k = 1; k <= types[i - 1][0] && j >= k * types[i - 1][1]; k ++)
//                     f[j] = (f[j] + f[j - k * types[i - 1][1]]) % MOD;
//         return f[t];
//     }
// };




/// FAILED!
// impl Solution {
//     pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
//         let _MOD = 1000_000_007;
//         let type_len = types.len();
//         let mut dp = vec![vec![0; target as usize]; type_len];
//         for i in 1..=target as usize {
//             if i as i32 / types[0][1] <= types[0][0] {
//                 dp[0][i-1] = 1;
//             } else {
//                 dp[0][i-1] = 0;
//             }
//         }
    
//         for i in 1..type_len {
//             for j in 1..=target as usize {
//                 if j >= i + 2 {
//                     if (j/(i+1)) as i32 <= types[i][0] {
//                         dp[i][j-1] = dp[i-1][j-1] + dp[i][j-2-i];
//                     } else {
//                         dp[i][j-1] = dp[i-1][j-1] + dp[i-1][j-2-i];
//                     }
//                 } else {
//                     dp[i][j-1] = dp[i-1][j-1];
//                 }
//                 println!("{:?}", dp);
//             }
//         }
        
//         (dp[type_len-1][target as usize -1] % _MOD) as i32
//     }
// }
