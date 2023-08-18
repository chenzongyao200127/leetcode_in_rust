// 97_Interleaving_String
// https://leetcode.cn/problems/interleaving-string/


// 给定三个字符串 s1、s2、s3，请你帮忙验证 s3 是否是由 s1 和 s2 交错 组成的。

// 两个字符串 s 和 t 交错 的定义与过程如下，其中每个字符串都会被分割成若干 非空 子字符串：

// s = s1 + s2 + ... + sn
// t = t1 + t2 + ... + tm
// |n - m| <= 1
// 交错 是 s1 + t1 + s2 + t2 + s3 + t3 + ... 或者 t1 + s1 + t2 + s2 + t3 + s3 + ...
// 注意：a + b 意味着字符串 a 和 b 连接。

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        Self::helper(&s1, 0, &s2, 0, &s3, 0)
    }

    fn helper(s1: &String, i: usize, s2: &String, j: usize, s3: &String, k: usize) -> bool {
        if k == s3.len() {
            return i == s1.len() && j == s2.len();
        }

        let mut valid = false;

        if i < s1.len() && s1.as_bytes()[i] == s3.as_bytes()[k] {
            valid = valid || Self::helper(s1, i + 1, s2, j, s3, k + 1);
        }

        if j < s2.len() && s2.as_bytes()[j] == s3.as_bytes()[k] {
            valid = valid || Self::helper(s1, i, s2, j + 1, s3, k + 1);
        }
        
        valid
    }
}



// 增加记忆化搜索
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut memo = vec![vec![None; s2.len() + 1]; s1.len() + 1];
        Self::helper(&s1, 0, &s2, 0, &s3, 0, &mut memo)
    }

    fn helper(s1: &String, i: usize, s2: &String, j: usize, s3: &String, k: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if k == s3.len() {
            return i == s1.len() && j == s2.len();
        }
        if let Some(res) = memo[i][j] {
            return res;
        }
        
        let mut valid = false;

        if i < s1.len() && s1.as_bytes()[i] == s3.as_bytes()[k] {
            valid = valid || Self::helper(s1, i + 1, s2, j, s3, k + 1, memo);
        }

        if !valid && j < s2.len() && s2.as_bytes()[j] == s3.as_bytes()[k] {
            valid = valid || Self::helper(s1, i, s2, j + 1, s3, k + 1, memo);
        }
        
        memo[i][j] = Some(valid);

        valid
    }
}

// DP
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        
        let (m, n, l) = (s1.len(), s2.len(), s3.len());
        let (s1_bytes, s2_bytes, s3_bytes) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] && s1_bytes[i - 1] == s3_bytes[i - 1];
        }

        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] && s2_bytes[j - 1] == s3_bytes[j - 1];
        }

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = (dp[i - 1][j] && s1_bytes[i - 1] == s3_bytes[i + j - 1]) ||
                           (dp[i][j - 1] && s2_bytes[j - 1] == s3_bytes[i + j - 1]);
            }
        }

        dp[m][n]
    }
}
