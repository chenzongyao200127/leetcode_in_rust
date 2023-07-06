// 2178_Maximum_Split_of_Positive_Even_Integers
// https://leetcode.cn/problems/maximum-split-of-positive-even-integers/

impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum % 2 != 0 {
            return vec![];
        }

        let mut tmp = 0;
        let mut res: Vec<i64> = vec![];
        while tmp < final_sum {
            tmp += 2;
            final_sum -= tmp;
            res.push(tmp);
        }
        *res.last_mut().unwrap() += final_sum;
        return res
    }
}