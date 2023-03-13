// 2383_Minimum_Hours_of_Training_to_Win_a_Competition
// https://leetcode.cn/problems/minimum-hours-of-training-to-win-a-competition/


impl Solution {
    pub fn min_number_of_hours(initial_energy: i32, initial_experience: i32, 
        energy: Vec<i32>, experience: Vec<i32>) -> i32 {
        
        let mut ans = 0;
        let sum_egy = energy.iter().sum::<i32>();
        if sum_egy >= initial_energy {
            ans += sum_egy+1-initial_energy;
        }
        let mut cur_exp = initial_experience;
        for i in 0..experience.len() {
            if cur_exp > experience[i] {
                cur_exp += experience[i];
            } else {
                ans += experience[i] + 1 - cur_exp;
                cur_exp += 2 * experience[i] + 1 - cur_exp;
            }
        }

        ans
    }
}