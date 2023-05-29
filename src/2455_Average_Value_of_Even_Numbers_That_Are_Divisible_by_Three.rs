// 2455_Average_Value_of_Even_Numbers_That_Are_Divisible_by_Three
// https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three/

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let nums: Vec<_> = nums.into_iter().filter(|&x| x%6==0).collect();
        if nums.len() == 0 {
            return 0;
        } else {
            nums.iter().sum::<i32>() / nums.len() as i32
        }
    }
}


impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, cnt) = nums.iter().filter(|x| *x % 6 == 0).fold((0, 0), |(sum, cnt), &x| (sum + x, cnt + 1) );
        if cnt == 0 { 0 } else { sum / cnt }
    }
}