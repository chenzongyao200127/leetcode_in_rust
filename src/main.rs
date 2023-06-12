
use std::collections::VecDeque;

pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    
    // Build the tree
    let mut g = vec![VecDeque::new(); n];
    for edge in &edges {
        let x = (edge[0] - 1) as usize;
        let y = (edge[1] - 1) as usize;
        g[x].push_back(y);
        g[y].push_back(x);
    }

    let mut ans = vec![0; n - 1];
    
    // Binary enumeration
    for mask in 3..(1 << n) {   // 需要至少两个点
        if mask & (mask - 1) == 0 {
            continue;
        }
        
        // 求树的直径
        let mut vis = 0;
        let mut diameter = 0;

        fn dfs(x: usize, g: &Vec<VecDeque<usize>>, mask: usize, vis: &mut usize, diameter: &mut usize) -> i32 {
            *vis |= 1 << x; // 标记 x 访问过

            let mut max_len = 0;
            for &y in &g[x] {
                if (*vis >> y & 1) == 0 && (mask >> y & 1) == 1 { // y 没有访问过且在 mask 中
                    let ml = dfs(y, g, mask, vis, diameter) + 1;
                    *diameter = (*diameter).max(max_len as usize + ml as usize);
                    max_len = max_len.max(ml);
                }
            }
            max_len
        }

        // 从一个在 mask 中的点开始递归
        dfs((mask as usize).trailing_zeros() as usize, &g, mask, &mut vis, &mut diameter); 

        if vis == mask {
            ans[diameter - 1] += 1;
        }
    }

    ans
}

fn main() {
    // count_subgraphs_for_each_diameter()
}
