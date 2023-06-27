// 47_Permutations_II


// 没有减枝
use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(com: &mut Vec<i32>, nums: &[i32], n: usize, ans: &mut HashSet<Vec<i32>>) {
            if com.len() == n {
                ans.insert(com.clone());
                return;
            }
    
            for (i, &num) in nums.iter().enumerate() {
                com.push(num);
                let new_nums = [&nums[..i], &nums[i + 1..]].concat();
                dfs(com, &new_nums, n, ans);
                com.pop();
            }
        }
    
        let mut ans = HashSet::new();
        let n = nums.len();
    
        dfs(&mut Vec::new(), &nums, n, &mut ans);
        ans.into_iter().collect()
    }
}


use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(com: &mut Vec<i32>, nums: &[i32], n: usize, ans: &mut HashSet<Vec<i32>>) {
            if com.len() == n {
                ans.insert(com.clone());
                return;
            }
    
            for i in 0..nums.len() {
                // 跳过重复的数字以避免重复的全排列
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                com.push(nums[i]);
                let new_nums = [&nums[..i], &nums[i + 1..]].concat();
                dfs(com, &new_nums, n, ans);
                com.pop();
            }
        }
    
        let mut nums = nums;
        nums.sort_unstable(); // 对 nums 排序
        let mut ans = HashSet::new();
        let n = nums.len();
    
        dfs(&mut Vec::new(), &nums, n, &mut ans);
        ans.into_iter().collect()
    }
}



impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(com: &mut Vec<i32>, nums: &[i32], n: usize, ans: &mut Vec<Vec<i32>>) {
            if com.len() == n {
                ans.push(com.clone());
                return;
            }

            for (i, &num) in nums.iter().enumerate() {
                if i > 0 && nums[i as usize] == nums[i as usize - 1] {
                    continue;
                }
                com.push(num);
                let new_nums = [&nums[..i], &nums[i + 1..]].concat();
                dfs(com, &new_nums, n, ans);
                com.pop();
            }
        }

        let mut ans = Vec::new();
        let n = nums.len();
        nums.sort_unstable();

        dfs(&mut Vec::new(), &nums, n, &mut ans);
        ans
    }
}




impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        Solution::fill(&mut res, &mut nums[..], 0);

        res
    }

    fn fill<'a>(res: &mut Vec<Vec<i32>>, nums: &mut [i32], index: usize) {
        if nums.len() - 1 == index {
            res.push(nums.to_vec());
            return;
        }

        let mut set = std::collections::HashSet::new();
        set.insert(nums[index]);
        Solution::fill(res, nums, index + 1);

        for i in (index + 1)..nums.len() {
            if !set.contains(&nums[i]) {
                set.insert(nums[i]);

                nums.swap(index, i);
                Solution::fill(res, nums, index + 1);
                nums.swap(index, i);
            }
        }
    }
}