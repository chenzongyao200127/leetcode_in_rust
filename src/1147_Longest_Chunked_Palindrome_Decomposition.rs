// 1147. Longest Chunked Palindrome Decomposition
// https://leetcode.cn/problems/longest-chunked-palindrome-decomposition/

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let mut left = 1;
        let mut prev_left = 0;

        let mut right = text.len() - 1;
        let mut pos_right = text.len();
    
        let mut ans = 1;
    
        while left <= right {
            if text[prev_left..left] == text[right..pos_right] && left <= right {
                if left == right {
                    ans += 1;
                } else {
                    ans += 2;
                }

                prev_left = left;
                pos_right = right;
                left += 1;
                right -= 1;
            } else {
                left += 1;
                right -= 1;
            }
        }
    
        ans
    }
}

