// 《算法分析与设计》第3次作业 
// 224769 陈宗耀
// 给出 N 个 1-9 的数字 (v_1,v_2,...,v_m)，不改变它们的相对位置，在中间加入K个乘号和(N-K-1)个加号， (括号随便加) 使最终结果尽量大。
// 因为乘号和加号一共就是 N-1 个了，所以恰好每两个相邻数字之间都有一个符号。并说明其具有优化子结构性质及子问题重叠性质。


impl Solution {
    fn max_expression_value(numbers: &[u32], k: usize) -> u32 {
        let n = numbers.len();
        let mut dp = vec![vec![vec![0; k + 1]; n + 1]; n + 1];
    
        for i in 0..n {
            dp[i][1][0] = numbers[i];
        }
    
        for j in 2..=n {
            for i in 0..=n - j {
                for kk in 0..=k {
                    let mut max_value = 0;
    
                    for p in 1..j {
                        if kk > 0 {
                            max_value = max_value.max(dp[i][p][kk - 1] * dp[i + p][j - p][0]);
                        } else {
                            max_value = max_value.max(dp[i][p][0] + dp[i + p][j - p][kk]);
                        }
                    }
    
                    dp[i][j][kk] = max_value;
                }
            }
        }
    
        dp[0][n][k]
    }
}


