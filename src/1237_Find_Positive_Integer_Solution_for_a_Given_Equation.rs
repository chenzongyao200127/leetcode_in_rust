/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation
 * struct CustomFunction;
 * impl CustomFunction {
 *    pub fn f(x:i32,y:i32)->i32{}
 * }
 */

// 60 ms 0.00%
impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        for x in 1..=1000 {
            let mut pre = 0;
            if ans.is_empty() {
                pre = 1000;
            } else {
                pre = ans.last().unwrap()[1]-1;
            }
            for y in 1..=pre {
                if customfunction.f(x, y) == z {
                    ans.push(vec![x,y]);
                    break;
                }                
            }
        }

        ans
    }
}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for x in 1..=1000 {
            for y in 1..=1000 {
                let g = customfunction.f(x, y);
                if g == z { result.push(vec![x, y]); }
                if g >= z { break; }
            }
        }
        result
    }
}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        for x in 1..=1000 {
            for y in 1..=1000 {
                if customfunction.f(x, y) == z {
                    ans.push(vec![x,y]);
                } else if customfunction.f(x, y) <= z {
                    continue;
                } else {
                    break;
                }
            }
        }

        ans
    }
}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        for x in 1..=1000 {
            let mut left = 1;
            let mut right = 1000;
            while left <= right {
                let mid = (left + right) / 2;
                if customfunction.f(x,mid) == z {
                    ans.push(vec![x, mid]);
                }
                if customfunction.f(x, mid) > z {
                    right = mid - 1; 
                } else {
                    left = mid + 1;
                }
            }
        }

        ans
    }
}