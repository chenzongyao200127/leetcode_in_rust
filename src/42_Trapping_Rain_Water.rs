// 42_Trapping_Rain_Water
// https://leetcode.cn/problems/trapping-rain-water/


// 可惜超时
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let &highst = height.iter().max().unwrap();
        let len = height.len();
        let mut wall = vec![vec![0; len]; highst as usize];
        for i in 0..highst as usize {
            for j in 0..len {
                if highst - i as i32 <= height[j] {
                    wall[i][j] = 1;
                }
            }
        }
        
        let mut ans = 0;
        for i in 0..highst as usize {
            let mut left = 0;
            let mut right = len-1;
            while left < right && wall[i][left] != 1 {
                left += 1;
            }
            while right > left && wall[i][right] != 1 {
                right -= 1;
            }
            for j in left+1..right {
                if wall[i][j] == 0 {
                    ans += 1;
                }
            }
        }
    
        ans
    }
}

// 双指针
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
    }
}