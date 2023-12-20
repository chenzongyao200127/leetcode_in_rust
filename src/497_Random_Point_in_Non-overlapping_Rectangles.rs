// 497_Random_Point_in_Non-overlapping_Rectangles
// https://leetcode.cn/problems/random-point-in-non-overlapping-rectangles/description/

use rand::Rng; // For random number generation
use std::iter::Iterator; // For iterator methods

struct Solution {
    cumulative_weights: Vec<f64>,
    my_rects: Vec<Vec<i32>>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let total_weight: f64 = rects
            .iter()
            .map(|rect| ((rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1)) as f64)
            .sum();
        let cumulative_weights: Vec<f64> = rects
            .iter()
            .map(|rect| ((rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1)) as f64 / total_weight)
            .scan(0.0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect();

        Solution {
            cumulative_weights,
            my_rects: rects,
        }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let pick = rng.gen::<f64>();
        let idx = self
            .cumulative_weights
            .iter()
            .position(|&weight| weight >= pick)
            .unwrap_or(0);

        let rect = &self.my_rects[idx];
        let x = rng.gen_range(rect[0]..=rect[2] + 1);
        let y = rng.gen_range(rect[1]..=rect[3] + 1);

        vec![x, y]
    }
}
