// 229. Majority Element II
// https://leetcode.cn/problems/majority-element-ii/

// 【笔记】摩尔投票法。该算法用于1/2情况，它说：“在任何数组中，出现次数大于该数组长度一半的值只能有一个。”
// 那么，改进一下用于1/3。可以着么说：“在任何数组中，出现次数大于该数组长度1/3的值最多只有两个。”
// 于是，需要定义两个变量。空间复杂度为O(1)。
// 摩尔投票法：https://mabusyao.iteye.com/blog/2223195
// 算法1/3改进：https://blog.csdn.net/weixin_42768679/article/details/81567231

// Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let major = nums.len()/3;
        let mut soldier_a = 0;
        let mut cnt_a = 0;
        let mut soldier_b = 0;
        let mut cnt_b = 0;
        for i in 0..nums.len() {
            if (cnt_a == 0 || nums[i] == soldier_a) && nums[i] != soldier_b {
                soldier_a = nums[i];
                cnt_a += 1;
            } else if cnt_b == 0 || nums[i] == soldier_b {
                soldier_b = nums[i];
                cnt_b += 1;
            } else {
                cnt_a -= 1;
                cnt_b -= 1;
            }
        }
        let mut ans = vec![];
        cnt_a = 0;
        cnt_b = 0;
        for n in nums {
            if n == soldier_a {
                cnt_a += 1;
            }
            if n == soldier_b {
                cnt_b += 1;
            }
        }
        if cnt_a > major {
            ans.push(soldier_a);
        }
        if cnt_b > major && soldier_a != soldier_b {
            ans.push(soldier_b);
        }
    
        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let major = nums.len()/3;
        let mut ans = vec![];
        for n in nums {
            map.entry(n).and_modify(|cnt| *cnt+=1).or_insert(1);
        }
        for (&k, &v) in map.iter() {
            if v > major {
                ans.push(k);
            }
        }

        ans
    }
}


impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut votes = vec![0;2];
        let mut vvs = vec![0usize;2];
        let mut types = 0;
        let mut flag :bool;
        for i in nums.iter() {
            flag =false;
            for (ij,j) in votes.iter().enumerate(){
                if i == j&& vvs[ij]>0{
                    vvs[ij]+=1;
                    flag = true;
                }
            }
            if !flag{
                if types == 2{
                    for j in vvs.iter_mut(){
                        *j-=1;
                        if *j ==0usize{
                            types -=1;
                        }
                    }
                }else {
                    for (ij,j) in vvs.iter_mut().enumerate(){
                        if *j<=0{
                            *j=1;
                            votes[ij] = *i;
                            types+=1;
                            break;
                        }
                    }
                }
            }
        }

        let times = nums.len()/3;
        let mut counts = vec![0usize;2];

        for i in 0..2{
            for k in nums.iter(){
                if *k == votes[i] && vvs[i] > 0 {
                    counts[i]+=1;
                }
            }
            if counts[i] > times{
                res.push(votes[i]);
            }

        }
        res
    }
}