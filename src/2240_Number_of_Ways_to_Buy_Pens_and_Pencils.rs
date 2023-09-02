// 2240_Number_of_Ways_to_Buy_Pens_and_Pencils
// https://leetcode.cn/problems/number-of-ways-to-buy-pens-and-pencils/

// 示例 1：
// 输入：total = 20, cost1 = 10, cost2 = 5
// 输出：9
// 解释：一支钢笔的价格为 10 ，一支铅笔的价格为 5 。
// - 如果你买 0 支钢笔，那么你可以买 0 ，1 ，2 ，3 或者 4 支铅笔。
// - 如果你买 1 支钢笔，那么你可以买 0 ，1 或者 2 支铅笔。
// - 如果你买 2 支钢笔，那么你没法买任何铅笔。
// 所以买钢笔和铅笔的总方案数为 5 + 3 + 1 = 9 种。

// 示例 2：
// 输入：total = 5, cost1 = 10, cost2 = 10
// 输出：1
// 解释：钢笔和铅笔的价格都为 10 ，都比拥有的钱数多，所以你没法购买任何文具。所以只有 1 种方案：买 0 支钢笔和 0 支铅笔。

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut total = total as i64;
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;
        let mut ans = 0;
        let mut cnt = 0;

        while cnt * cost1 <= total {
            total -= cost1;
            if total < cost2 {
                ans += 1
            } else {
                ans += (total / cost2 + 1)
            }
            cnt += 1;
            total += cost1;
        } 

        ans
    }
}