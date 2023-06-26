// 77_Combinations
// https://leetcode.cn/problems/combinations/

// 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
// 你可以按 任何顺序 返回答案。

// 很多不必要的clone操作
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let tmp = vec![];
        Self::dfs(tmp, 1, &n, &k, &mut ans);
    
        return ans;
    }
    
    fn dfs(tmp: Vec<i32>, x: usize, n: &i32, k: &i32, ans: &mut Vec<Vec<i32>>) {
        if tmp.len() == *k as usize {
            if !ans.contains(&tmp) {
                ans.push(tmp.clone());
            }
            return;
        }
    
        if x > *n as usize {
            return;
        } 
    
        let tmp2 = tmp.clone();
        Self::dfs(tmp2, x+1, n, k, ans);
    
        let mut tmp1 = tmp.clone();
        tmp1.push(x as i32);
        Self::dfs(tmp1, x+1, n, k, ans);
        tmp1.pop();
    }
}




// 改进


// 改进
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current_combination: Vec<i32> = vec![];
        Self::dfs(&mut current_combination, 1, n, k, &mut result);
        result
    }

    fn dfs(
        current_combination: &mut Vec<i32>,
        start: i32,
        n: i32,
        k: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        // 通过剪枝，可以避免搜索不必要的组合，从而提高代码的执行效率。
        if (current_combination.len() + (n - start + 1) as usize) < k as usize {
            return;
        }

        if current_combination.len() == k as usize {
            result.push(current_combination.clone());
            return;
        }

        if start > n {
            return;
        }

        Self::dfs(current_combination, start + 1, n, k, result);

        current_combination.push(start);
        Self::dfs(current_combination, start + 1, n, k, result);
        current_combination.pop();
    }
}



impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(cur: i32, n: i32, k:i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if tmp.len() + ((n - cur + 1) as usize) < k as usize {
                return;
            }
            // 记录合法的答案
            if tmp.len() == k as usize {
                ans.push(tmp.to_vec());
                return;
            }
            // 考虑选择当前位置
            tmp.push(cur);
            dfs(cur + 1, n, k, tmp, ans);
            tmp.pop();
            // 考虑不选择当前位置
            dfs(cur + 1, n, k, tmp, ans);
        }
        let mut tmp: Vec<i32> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        dfs(1, n, k, &mut tmp, &mut ans);
        ans
    }
}