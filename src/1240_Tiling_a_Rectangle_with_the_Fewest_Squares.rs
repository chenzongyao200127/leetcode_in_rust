// 1240_Tiling_a_Rectangle_with_the_Fewest_Squares
// https://leetcode.cn/problems/tiling-a-rectangle-with-the-fewest-squares/

struct Solution;

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        // 深度优先搜索
        fn dfs(x: i32, y: i32, cnt: i32, ans: &mut i32, n: i32, m: i32, rect: &mut Vec<Vec<bool>>) {
            // 如果当前计数大于等于最小值，则提前返回
            if cnt >= *ans {
                return;
            }
            // 如果 x 达到 n，则更新最小值
            if x >= n {
                *ans = cnt;
                return;
            }
            // 如果 y 达到 m，则检查下一行
            if y >= m {
                dfs(x + 1, 0, cnt, ans, n, m, rect);
                return;
            }
            // 如果当前位置已覆盖，则尝试下一个位置
            if rect[x as usize][y as usize] {
                dfs(x, y + 1, cnt, ans, n, m, rect);
                return;
            }

            // 检查可行的正方形大小
            let mut k = std::cmp::min(n - x, m - y);
            while k >= 1 && is_available(x, y, k, n, m, &rect) {
                fill_up(x, y, k, true, rect);
                dfs(x, y + k, cnt + 1, ans, n, m, rect);
                fill_up(x, y, k, false, rect);
                k -= 1;
            }
        }

        // 检查给定的正方形是否可用
        fn is_available(x: i32, y: i32, k: i32, n: i32, m: i32, rect: &[Vec<bool>]) -> bool {
            for i in 0..k {
                for j in 0..k {
                    if rect[(x + i) as usize][(y + j) as usize] == true {
                        return false;
                    }
                }
            }
            true
        }

        // 填充或取消填充给定的正方形
        fn fill_up(x: i32, y: i32, k: i32, val: bool, rect: &mut Vec<Vec<bool>>) {
            for i in 0..k {
                for j in 0..k {
                    rect[(x + i) as usize][(y + j) as usize] = val;
                }
            }
        }

        // 初始化最小瓷砖数量和矩形
        let mut ans = std::cmp::max(n, m);
        let mut rect = vec![vec![false; m as usize]; n as usize];
        
        // 从 (0, 0) 开始深度优先搜索
        dfs(0, 0, 0, &mut ans, n, m, &mut rect);
        
        ans
    }
}