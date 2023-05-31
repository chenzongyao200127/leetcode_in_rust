// 1130_Minimum_Cost_Tree_From_Leaf_Values
// https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values/submissions/

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stk = vec![arr[0]];

        for &n in arr.iter().skip(1) {
            let mut cur = n;
            while !stk.is_empty() && *stk.last().unwrap() <= cur {
                let top = stk.pop().unwrap();
                if stk.is_empty() || *stk.last().unwrap() > cur {
                    cur = cur.max(top);
                    ans += cur * top;
                } else {
                    ans += top * stk.last().unwrap();
                }
            }
            stk.push(cur);
        }

        stk.windows(2).rev().for_each(|x| {
            ans += x[0] * x[1];
        });
        
        ans
    }
}


impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        
        let mut max = vec![vec![0; n]; n];
        for i in 0..n {
            let mut acc = 0;
            for j in i..n {
                acc = acc.max(arr[j]);
                max[i][j] = acc;
            }
        }

        let mut f = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            f[i][i] = 0;
        }
        for l in 1..n {
            for i in 0..n {
                let j = i + l;
                if j >= n { break; }
                for k in i..j {
                    f[i][j] = f[i][j].min(f[i][k] + f[k+1][j] + max[i][k].saturating_mul(max[k+1][j]));
                }
            }
        }

        f[0][n-1]
    }
}