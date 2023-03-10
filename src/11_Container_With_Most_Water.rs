// 11_Container_With_Most_Water
// https://leetcode.cn/problems/container-with-most-water/

// 暴力超时
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let len = height.len();
//         let mut ans = 0;
//         for i in 0..len-1 {
//             for j in i+1..len {
//                 ans = ans.max((j-i) as i32 * (height[i].min(height[j])));
//             }
//         }
//         ans
//     }
// }


impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut ans = 0;
        let mut left = 0;
        let mut right = len-1;
        while left < right {
            ans = ans.max((right-left) as i32 * (height[left].min(height[right])));
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let width = (right - left) as i32;
            if height[left] < height[right] {
                let area = height[left] * width;
                max_area = max_area.max(area);
                left += 1;
            } else {
                let area = height[right] * width;
                max_area = max_area.max(area);
                right -= 1;
            }
        }
        max_area
    }
}