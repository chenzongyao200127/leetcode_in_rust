// 167. Two Sum II - Input Array Is Sorted
// https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/


// 你所设计的解决方案必须只使用常量级的额外空间。
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..numbers.len()-1 {
            for j in i+1..numbers.len() {
                if numbers[i] + numbers[j] == target {
                    ans.push(i as i32 + 1);
                    ans.push(j as i32 + 1);
                }
            }
        }
        ans
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..numbers.len() {
            let lookfor = target - numbers[i];
            if let Some(&cnt) = map.get(&lookfor) {
                ans.push(i as i32 + 1);
                ans.push(cnt + 1);
            } else {
                map.insert(numbers[i], i as i32);
            }
        }

        ans.sort_unstable();
        ans
    }
}

// 为什么双指针往中间移动时，不会漏掉某些情况呢？要解答这个问题，我们要从 缩减搜索空间 的角度思考这个解法。下面我将以文字和图片两种方式进行讲解。
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0 as usize;
        let mut r = numbers.len()-1;
        loop {
            if numbers[l] + numbers[r] > target {
                r -= 1;
            } else if numbers[l] + numbers[r] < target {
                l += 1;
            } else {
                break;
            }
        }
        return vec![l as i32 + 1, r as i32 + 1];
    }
}
