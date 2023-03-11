// 1705_Find_Longest_Subarray_LCCI
// https://leetcode.cn/problems/find-longest-subarray-lcci/

use std::collections::HashMap;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut new_arr = vec![];
        for s in array.iter() {
            if let Ok(_) = s.parse::<i32>() {
                new_arr.push(1);
            } else {
                new_arr.push(-1);
            }
        }
        let mut max = i32::MIN;
        let mut max_end = 0 as usize;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut pre_sum = 0;
        for i in 0..new_arr.len() {
            pre_sum += new_arr[i];
            if map.contains_key(&pre_sum) {
                let len = i as i32 - map.get(&pre_sum).unwrap();
                if len > max {
                    max = len;
                    max_end = i;
                }
            } else {
                map.insert(pre_sum, i as i32);
            }
        }

        let mut ans = vec![];
        // println!("{:?}", (max, max_end));
        for i in max_end+1-max as usize..=max_end {
            ans.push(array[i].clone());
        }
        ans
    }
}


impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let (mut s, mut begin, mut len) = (0, 0, 0);
        let mut map = HashMap::new();
        map.insert(0, 0);
        for (i, v) in array.iter().enumerate() {
            s += match v.as_bytes()[0] {
                b'0'..=b'9' => 1,
                _ => -1
            };
            let c = map.entry(s).or_insert(i+1);
            let l = i+1 - *c;
            if l > len {
                begin = *c;
                len = l;
            }
        }
        array.into_iter().skip(begin).take(len).collect()
    }
}
