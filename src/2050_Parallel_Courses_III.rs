// 2050. 并行课程 III
// https://leetcode.cn/problems/parallel-courses-iii/


// 给你一个整数 n ，表示有 n 节课，课程编号从 1 到 n 。同时给你一个二维整数数组 relations ，
// 其中 relations[j] = [prevCoursej, nextCoursej] ，
// 表示课程 prevCoursej 必须在课程 nextCoursej 之前 完成（先修课的关系）。
// 同时给你一个下标从 0 开始的整数数组 time ，其中 time[i] 表示完成第 (i+1) 门课程需要花费的 月份 数。

// 请你根据以下规则算出完成所有课程所需要的 最少 月份数：

// 如果一门课的所有先修课都已经完成，你可以在 任意 时间开始这门课程。
// 你可以 同时 上 任意门课程 。
// 请你返回完成所有课程所需要的 最少 月份数。

// 注意：测试数据保证一定可以完成所有课程（也就是先修课的关系构成一个有向无环图）。
use std::collections::HashMap;
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut prev: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
        for rela in relations {
            prev[rela[1] as usize].push(rela[0].clone());
        }
        
        let mut memo: HashMap<i32, i32> = HashMap::new();
        
        #[inline]
        fn dfs(i: i32, memo: &mut HashMap<i32, i32>, prev: &Vec<Vec<i32>>, time: &Vec<i32>) -> i32 {
            if let Some(v) = memo.get(&i) {
                return *v
            }

            let mut cur = 0;
            for pre in prev[i as usize].iter() {
                cur = cur.max(dfs(*pre, memo, prev, time));
            }
            cur += time[i as usize - 1];
            memo.insert(i, cur);
            return cur
        }

        let mut ans = 0;
        (1..=n).into_iter().for_each(|x| ans = ans.max(dfs(x, &mut memo, &prev, &time)));
        ans
    }
}