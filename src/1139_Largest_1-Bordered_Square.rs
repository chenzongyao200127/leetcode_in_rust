// 1139. Largest 1-Bordered Square
// https://leetcode.cn/problems/largest-1-bordered-square/

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(grid.len(),grid[0].len());
        let mut dp=vec![vec![vec![0;2];n+1];m+1];
        for (i,row) in grid.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==1{
                    dp[i+1][j+1][0]=dp[i+1][j][0]+1;
                    dp[i+1][j+1][1]=dp[i][j+1][1]+1;
                }
            }
        }
        let mut max_side=0;
        for i in 1..=m{
            for j in 1..=n{
                let mut cur_side=dp[i][j][0].min(dp[i][j][1]);
                if cur_side<=max_side{
                    continue
                }
                while cur_side>max_side{
                    if dp[i-cur_side+1][j][0]>=cur_side && dp[i][j-cur_side+1][1]>=cur_side{
                        max_side=cur_side;
                        break
                    }
                    cur_side-=1;
                }
            }
        }
       ( max_side*max_side) as _

    }
}

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let (row_num, col_num) = (grid.len(), grid[0].len());
        let mut max_border = 0;

        let mut left = vec![vec![0; col_num+1]; row_num+1];
        let mut up = vec![vec![0; col_num+1]; row_num+1];

        for i in 1..=row_num {
            for j in 1..=col_num {
                // 当第i行第j列数字为1时，更新记录矩阵
                if grid[i-1][j-1] == 1 {
                    left[i][j] = left[i][j-1] + 1;
                    up[i][j] = up[i-1][j] + 1;
                    let mut border = left[i][j].min(up[i][j]);
                    // 验证另外两边是否符合约束，即左边与上边
                    while left[i - border as usize + 1][j] < border || up[i][j - border as usize + 1] < border {
                        border -= 1;
                    }
                    // 将本次循环得到的新边长与已有最大边长做对比
                    max_border = max_border.max(border);
                } 
            }
        }

        max_border * max_border
}
}