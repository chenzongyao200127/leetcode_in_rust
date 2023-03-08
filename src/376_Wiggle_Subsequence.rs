// 376_Wiggle_Subsequence
// https://leetcode.cn/problems/wiggle-subsequence/

// 376. 摆动序列
// 如果连续数字之间的差严格地在正数和负数之间交替，则数字序列称为 摆动序列 。第一个差（如果存在的话）可能是正数或负数。仅有一个元素或者含两个不等元素的序列也视作摆动序列。
// 例如， [1, 7, 4, 9, 2, 5] 是一个 摆动序列 ，因为差值 (6, -3, 5, -7, 3) 是正负交替出现的。
// 相反，[1, 4, 7, 2, 5] 和 [1, 7, 4, 5, 5] 不是摆动序列，第一个序列是因为它的前两个差值都是正数，第二个序列是因为它的最后一个差值为零。
// 子序列 可以通过从原始序列中删除一些（也可以不删除）元素来获得，剩下的元素保持其原始顺序。
// 给你一个整数数组 nums ，返回 nums 中作为 摆动序列 的 最长子序列的长度 。

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        if nums.len() == 2 {
            if nums[0] != nums[1] {
                return 2;
            } else {
                return 1;
            }
        }

        let mut ans = vec![nums[0]];
        let mut diff = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[0] {
                ans.push(nums[i]);
                diff = i;
                break;
            }
        }
        
        if diff == 0 {
            return 1;
        }

        for i in diff..nums.len() {
            if (nums[i] - ans[ans.len()-1]) * (ans[ans.len()-1] - ans[ans.len()-2]) < 0 {
                ans.push(nums[i]);
            } else {
                if (ans[ans.len()-1] - ans[ans.len()-2]) > 0 {
                    if ans[ans.len()-1] >= nums[i] {
                        continue;
                    } else {
                        ans.pop();
                        ans.push(nums[i]);
                    }
                } else {
                    if ans[ans.len()-1] <= nums[i] {
                        continue;
                    } else {
                        ans.pop();
                        ans.push(nums[i]);
                    }
                }
            }
        }
        // println!("{:?}", ans);

        ans.len() as i32
    }
}



impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut res = 1;
        let mut pre_diff = 0;
        for i in 0..nums.len() - 1 {
            let cur_diff = nums[i + 1] - nums[i];
            if (pre_diff <= 0 && cur_diff > 0) || (pre_diff >= 0 && cur_diff < 0) {
                res += 1;
                pre_diff = cur_diff;
            }
        }
        res
    }
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let (mut up, mut down) = (1, 1);
        nums.windows(2).for_each(|x| {
            if x[0] > x[1] {
                down = down.max(up + 1);
            } else if x[0] < x[1] {
                up = up.max(down + 1);
            }
        });
        up.max(down)
    }
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let (mut down, mut up) = (1, 1);
        for i in 1..nums.len() {
            // i - 1 为峰顶
            if nums[i] < nums[i - 1] {
                down = down.max(up + 1);
            }
            // i - 1 为峰谷
            if nums[i] > nums[i - 1] {
                up = up.max(down + 1);
            }
        }
        down.max(up)
    }
}