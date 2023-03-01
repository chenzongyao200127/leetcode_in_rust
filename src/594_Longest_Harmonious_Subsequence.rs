// 594. Longest Harmonious Subsequence
// https://leetcode.cn/problems/longest-harmonious-subsequence/

use std::collections::HashMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut ans = 0;
        for num in nums {
            map.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }
        for (k,v) in map.iter() {
            if map.contains_key(&(*k+1)) {
                ans = ans.max(*v + map.get(&(*k+1)).unwrap())
            }
        }
    
        ans as i32
    }
}


impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
    if nums.len()==1{
        return 0;
    }
    let mut nums=nums;
    nums.sort();//排序
    let mut max=0;
    for i in 1..nums.len(){
        if nums[i]-nums[i-1]==1{//找到相邻差1的值
            let mut a=0;
            let mut b=0;
            while (i-1-a) as i32>=0&&nums[i-1-a]==nums[i-1] {//相邻左边相同的数
                a+=1;
            }
            while i+b<=nums.len()-1&&nums[i+b]==nums[i] {//相邻右边相同的数
                b+=1;
            }
            max=max.max(a+b);
        }
    }
    max as i32
    }
}