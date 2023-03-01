// 1326. Minimum Number of Taps to Open to Water a Garden
// https://leetcode.cn/problems/minimum-number-of-taps-to-open-to-water-a-garden/


impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut covers = vec![];
        ranges.into_iter().enumerate().for_each(|(i, x)| {
            let left = 0.max(i as i32 - x);
            let right = n.min(i as i32 + x);
            covers.push((left, right));        
        });
        covers.sort_by_key(|(left, _)| left.abs());
        // println!("{:?}", covers);
    
        let mut cur_cover_right = -1;
        let mut ans = 0;
        for i in 0..=n {
            if cur_cover_right >= n {
                break;
            }
            if cur_cover_right > i {
                continue;
            }
            let mut cover_i = vec![];
            covers.iter().for_each(|(left, right)| {
                if *left <= i && *right >= i+1 {
                    cover_i.push((left, right));
                }
            });
            cover_i.sort_by_key(|(_, right)| right.abs());
            // println!("cover_i:{:?}", cover_i);
            if cover_i.len() == 0 {
                return -1;
            }
            cur_cover_right = (cover_i[cover_i.len()-1].1).clone();
            ans += 1;
            // println!("{:?}", cur_cover_right);
        }
    
        if cur_cover_right >= n {
            return ans;
        } else {
            -1
        }
    }
}


impl Solution {
    /// 排序+贪心
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut help: Vec<(i32, i32)> = ranges.into_iter().enumerate().map(|(i, r)| {
            (i as i32 - r, i as i32 + r)
        }).collect();
        help.sort_unstable();
        let mut result = 0;
        let mut end = 0;
        let mut i = 0;
        while end < n {
            let mut right = end;
            while i < help.len() && help[i].0 <= end {
                right = right.max(help[i].1);
                i += 1;
            }
            if right == end { break; }
            result += 1;
            end = right;
        }
        if end < n { -1 } else { result }
    }
}