// 1262_Greatest_Sum_Divisible_by_Three
// https://leetcode.cn/problems/greatest-sum-divisible-by-three/

// Represent the state as DP[pos][mod]: maximum possible sum starting in the position "pos" in the array 
// where the current sum modulo 3 is equal to mod.
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 3]; nums.len() + 1];
        dp[0][1] = i32::MIN;
        dp[0][2] = i32::MIN;
        
        for i in 1..=nums.len() {
            let cmod = (nums[i - 1] % 3) as usize;
            if cmod == 0 {
                dp[i][0] = dp[i - 1][0] + nums[i - 1];
                dp[i][1] = dp[i - 1][1] + nums[i - 1];
                dp[i][2] = dp[i - 1][2] + nums[i - 1];
            } else if cmod == 1 {
                dp[i][0] = dp[i - 1][0].max(dp[i - 1][2] + nums[i - 1]);
                dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + nums[i - 1]);
                dp[i][2] = dp[i - 1][2].max(dp[i - 1][1] + nums[i - 1]);
            } else {
                dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + nums[i - 1]);
                dp[i][1] = dp[i - 1][1].max(dp[i - 1][2] + nums[i - 1]);
                dp[i][2] = dp[i - 1][2].max(dp[i - 1][0] + nums[i - 1]);
            }
            // println!("{:?}", dp);
        }
        return dp[nums.len()][0];
    }
}




impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        // dp[0][i]:  nums[..i] 能被 3 整除的最大和
        // dp[1][i]:  nums[..i] 除以 3 余 1 的最大和
        // dp[2][i]:  nums[..i] 除以 3 余 2 的最大和
        let mut dp = vec![vec![i32::MIN; nums.len()]; 3];
        let remaind = nums[0] as usize % 3;
        dp[remaind][0] = nums[0];
        for (i, n) in nums.into_iter().enumerate().skip(1) {
            let remaind = n as usize % 3;
            match remaind {
                0 => {
                    dp[0][i] = 0.max(dp[0][i - 1]) + n;
                    dp[1][i] = dp[1][i - 1] + n;
                    dp[2][i] = dp[2][i - 1] + n;
                }
                1 => {
                    dp[0][i] = (dp[2][i - 1] + n).max(dp[0][i - 1]);
                    dp[1][i] = (dp[0][i - 1] + n).max(dp[1][i - 1]).max(n);
                    dp[2][i] = (dp[1][i - 1] + n).max(dp[2][i - 1]);
                }
                2 => {
                    dp[0][i] = (dp[1][i - 1] + n).max(dp[0][i - 1]);
                    dp[1][i] = (dp[2][i - 1] + n).max(dp[1][i - 1]);
                    dp[2][i] = (dp[0][i - 1] + n).max(dp[2][i - 1]).max(n);
                }
                _ => unreachable!(),
            }
        }
        dp[0].last().cloned().map(|v| v.max(0)).unwrap()
    }
}


// DP
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, i32::MIN, i32::MIN];
        
        for num in nums {
            let mut temp_dp = dp.clone();
            for i in 0..3 {
                let cmod = ((dp[i] + num) % 3 + 3) as usize % 3;
                temp_dp[cmod] = temp_dp[cmod].max(dp[i] + num);
            }
            dp = temp_dp;
        }
        dp[0]
    }
}