use std::collections::HashSet;

pub fn main() {
    let ans = wiggle_max_length(vec![0,0,0]);
    assert_eq!(ans, 6);
}


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