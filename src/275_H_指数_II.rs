// 275_H_指数_II
// https://leetcode.cn/problems/h-index-ii/description/

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_by(|a, b| b.cmp(a));
    
        let mut l = 0;
        let mut r = citations.len() as i32 - 1;
    
        while l <= r {
            let mid = l + (r - l) / 2;
            
            if mid + 1 <= citations[mid as usize] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    
        r + 1
    }
}