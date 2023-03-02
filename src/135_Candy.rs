// 135. 分发糖果
// https://leetcode.cn/problems/candy/


impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut kids = ratings;
        kids.sort_unstable();
        let mut ans = ratings.len();
        let mut pre = kids[0];
        for i in 1..ratings.len() {
            if kids[i] > pre {
                ans += 1;
            }
            pre = kids[i];
        }
        ans
    }
}