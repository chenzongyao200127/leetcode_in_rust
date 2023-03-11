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

/// 单调栈
/// 还是不熟悉，看着思路都没写出来....
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stk: Vec<usize>= vec![];
        let mut ans = 0;

        for i in 0..height.len() {
            while !stk.is_empty() && height[stk[stk.len()-1]] < height[i] {
                let top = stk.pop().unwrap();
                if stk.is_empty() {
                    break;
                }
                let left = stk[stk.len()-1];
                ans += (i-left-1)*((height[i].min(height[left])-height[top]) as usize);
            }
            stk.push(i);
        }

        ans as i32
    }
}