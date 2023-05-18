// 《算法分析与设计》第3次作业 
// 224769 陈宗耀
// 给定一长度为$N$的整数序列(a_1,a_2,...,a_n) ，将其划分成多个子序列 (此问题中子序列是连续的一段整数) ，
// 满足每个子序列中整数的和不大于一个数B，设计一种划分方法，最小化所有子序列中最大值的和。


impl Solution {
    fn minimize_max_subsequence_sum(numbers: Vec<usize>, B: usize) -> usize {
        let len = numbers.len();
        let mut dp = vec![0 as usize; len+1];
    
        dp[1] = numbers[0];
    
        for i in 2..=len {
            dp[i] = usize::MAX;
        }
    
        for i in 1..=len {
            let mut max_value = numbers[i-1]; // 当前子序列的最大值
            let mut sum_value = 0;            // 当前子序列的和
            for j in (1..=i).rev() {
                max_value = max_value.max(numbers[j-1]);
                sum_value += numbers[j-1];
                if sum_value > B {
                    break;
                }
                dp[i] = dp[i].min(dp[j-1] + max_value); // 将前i个元素划分的最优解可以通过将前j-1个元素划分的最优解加上当前子序列的最大值得到
            }
        }

        dp[len]
    }
}