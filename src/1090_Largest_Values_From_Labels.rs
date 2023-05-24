// 1090_Largest_Values_From_Labels
// https://leetcode.cn/problems/largest-values-from-labels/

impl Solution {
    pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, mut num_wanted: i32, use_limit: i32) -> i32 {
        let mut values: Vec<_> = values.iter().zip(labels.iter()).collect();
        values.sort_by(|a, b| (b.0).cmp(&(a.0)));
        let mut cnt = vec![0; 100000 as usize];
        let mut res_cnt = vec![0; 100000 as usize];
        labels.iter().for_each(|&x| cnt[x as usize] += 1);
        let mut ans = 0;
        values.iter().for_each(|&(&v, &l)| {
            if num_wanted == 0 {
                return;
            }
            if res_cnt[l as usize] < use_limit && cnt[l as usize] > 0 {
                ans += v;
                res_cnt[l as usize] += 1;
                num_wanted -= 1;
                cnt[l as usize] -= 1;
            }
        });
        
        ans
    }
}


impl Solution {
    pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, mut num_wanted: i32, use_limit: i32) -> i32 {
        let mut pairs: Vec<_> = values.into_iter().zip(labels.into_iter()).collect();
        pairs.sort_unstable_by_key(|&(v, _)| -v);
        
        let mut cnt = vec![0; 100000];
        let mut ans = 0;
        
        for (v, l) in pairs {
            if cnt[l as usize] < use_limit {
                ans += v;
                cnt[l as usize] += 1;
                num_wanted -= 1;
                if num_wanted == 0 {
                    break;
                }
            }
        }
        
        ans
    }
}