// 1423_可获得的最大点数
// https://leetcode.cn/problems/maximum-points-you-can-obtain-from-cards/


impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize; // Convert k to usize for indexing
        let total_points: i32 = card_points.iter().sum();
        let window_size = card_points.len() - k;
        let mut min_subarray_sum = card_points.iter().take(window_size).sum::<i32>();
        let mut current_sum = min_subarray_sum;

        for i in 0..k {
            // Slide the window by one position
            current_sum = current_sum - card_points[i] + card_points[i + window_size];
            min_subarray_sum = min_subarray_sum.min(current_sum);
        }

        total_points - min_subarray_sum
    }
}


impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        
        // DP arrays to store the maximum score when taking cards from the beginning or the end
        let mut dp_front = vec![0; k + 1];
        let mut dp_back = vec![0; k + 1];

        // Calculate the total points for the first k cards
        for i in 1..=k {
            dp_front[i] = dp_front[i - 1] + card_points[i - 1];
        }

        // Calculate the total points for the last k cards
        for i in 1..=k {
            dp_back[i] = dp_back[i - 1] + card_points[n - i];
        }

        // Max score could be taking all from front, all from back, or some from both
        let mut max_score = 0;
        for i in 0..=k {
            max_score = max_score.max(dp_front[i] + dp_back[k - i]);
        }

        max_score
    }
}


impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut s = card_points.iter().take(k).sum::<i32>();
        let mut ans = s;
        for i in 1..=k {
            s += card_points[card_points.len() - i] - card_points[k - i];
            ans = ans.max(s);
        }
        ans
    }
}
