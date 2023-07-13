// 399_Evaluate_Division
// https://leetcode.cn/problems/evaluate-division/description/


/// 在代码中，首先使用 HashMap 将变量名映射为数字，然后使用并查集维护变量之间的关系。
/// 在并查集中，每个变量对应一个节点，节点之间的关系由权重表示。
/// 在合并操作中，将两个变量的节点合并为一个，并更新权重。
/// 在查找操作中，查找两个变量的根节点，并计算它们之间的权重比值。
use std::collections::HashMap;

// 一边查询一边修改结点指向是并查集的特色。
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        // 初始化并查集的父节点和权重
        let mut parent: Vec<usize> = (0..equations.len() * 2).collect();
        let mut weight = vec![1.0_f64; equations.len() * 2];

        // 定义并查集中的合并操作
        fn union(parent: &mut Vec<usize>, weight: &mut Vec<f64>, idx1: usize, idx2: usize, val: f64) {
            // 找到 idx1 和 idx2 的根节点
            let x = find(parent, weight, idx1);
            let y = find(parent, weight, idx2);
            if x == y {
                // 如果根节点相同，无需合并
                return;
            }
            // 合并 idx1 和 idx2 的根节点，同时更新权重
            parent[x] = y;
            weight[x] = val * weight[idx2] / weight[idx1];
        }

        // 定义并查集中的查找操作
        fn find(parent: &mut Vec<usize>, weight: &mut Vec<f64>, mut idx: usize) -> usize {
            if idx != parent[idx] {
                // 递归查找根节点，并同时更新权重
                let p = find(parent, weight, parent[idx]);
                weight[idx] *= weight[parent[idx]];
                parent[idx] = p;
            }
            parent[idx]
        }

        let mut idx = 0;
        let mut map = HashMap::new();
        // 遍历方程式，将变量名映射为数字，并执行合并操作
        for (e, &v) in equations.iter().zip(values.iter()) {
            for e in [&e[0], &e[1]] {
                if map.get(&e).is_none() {
                    // 如果变量名没有被映射为数字，将其加入映射表，并分配一个数字
                    map.insert(e, idx);
                    idx += 1;
                }
            }
            // 执行合并操作
            union(&mut parent, &mut weight, map[&e[0]], map[&e[1]], v);
        }

        let mut ans = vec![];
        // 遍历查询，执行查找操作
        for ele in queries {
            if map.get(&ele[0]).is_none() || map.get(&ele[1]).is_none() {
                // 如果查询中存在未知的变量名，无法计算，返回 -1.0
                ans.push(-1.0);
                continue;
            }
            let idx1 = find(&mut parent, &mut weight, map[&ele[0]]); 
            let idx2 = find(&mut parent, &mut weight, map[&ele[1]]);
            if idx1 != idx2 {
                // 如果两个变量不在同一个集合中，无法计算，返回 -1.0
                ans.push(-1.0);
            } else {
                // 计算两个变量的比值，并加入答案数组
                ans.push(weight[map[&ele[0]]] / weight[map[&ele[1]]]);
            }
        }
        ans
    }
}