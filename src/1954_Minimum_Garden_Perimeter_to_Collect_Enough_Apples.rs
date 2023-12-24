// 1954_Minimum_Garden_Perimeter_to_Collect_Enough_Apples
// https://leetcode.cn/problems/minimum-garden-perimeter-to-collect-enough-apples/description/

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut n = 1;
        while 2 * n * (n + 1) * (2 * n + 1) < needed_apples {
            n += 1
        }
        return n * 8;
    }
}
