// 1963_Minimum_Number_of_Swaps_to_Make_the_String_Balanced
// https://leetcode.cn/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/?envType=daily-question&envId=2025-03-17

// Iterate over the string and keep track of the number of opening and closing brackets on each step.
// If the number of closing brackets is ever larger, you need to make a swap.
// Swap it with the opening bracket closest to the end of s.

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut res = 0;
        let mut sub = 0;
        for c in s.chars() {
            sub += if c == ']' { 1 } else { -1 };
            res = res.max(sub);
        }
        res + 1 >> 1
    }
}
