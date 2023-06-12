// 526_Beautiful_Arrangement
// https://leetcode.cn/problems/beautiful-arrangement/

impl Solution {
    // 简单的回溯
    pub fn count_arrangement(n: i32) -> i32 {
        // 设置一个标记数组valid[i]用来表示数字i是否可用, 注意因为题目中都是以1起始, 所以这里我多设置了1位, 以便能够取到valid[n]
        Solution::bt(1, n as usize, &mut vec![true; n as usize + 1])
    }
    
    // 辅助函数: 回溯
    fn bt(idx: usize, n: usize, valid: &mut Vec<bool>) -> i32 {
        match idx > n {
            true => 1, // 基本情况, 如果我们当前考虑到第n+1个数字(成功的填完了前n个数字), 表明我们找到了一种填充方案, 直接返回1
            // 其它情况, 我们要尝试填充所有符合当前位idx填充条件的数字num
            _ => (1..=n).fold(0, |ans, num| ans + if valid[num] && (num % idx == 0 || idx % num == 0) {
                    valid[num] = false; // 注意选完之后这个值要标记为不可用
                    let res = Solution::bt(idx+1, n, valid);
                    valid[num] = true; // 回溯完将状态复原
                    res
                } else {0}),
        }
    }
}


impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut vis = vec![false; n+1];
        let mut matches = vec![vec![]; n+1];
        for i in 1..=n {
            for j in 1..=n {
                if i%j == 0 || j%i == 0 {
                    matches[i].push(j);
                }
            }
        }

        fn bt(i: usize, vis: &mut Vec<bool>, matches: &Vec<Vec<usize>>, ans: &mut i32) {
            if i == vis.len() {
                *ans += 1;
                return
            }

            for &j in &matches[i] {
                if !vis[j] {
                    vis[j] = true;
                    bt(i+1, vis, matches, ans);
                    vis[j] = false;
                }
            }
        }
        
        let mut ans = 0;
        bt(1, &mut vis, &matches, &mut ans);
        ans
    }
}