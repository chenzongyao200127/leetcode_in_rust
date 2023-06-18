// 1254_Number_of_Closed_Islands
// https://leetcode.cn/problems/number-of-closed-islands/


// 输入：grid = [[1,1,1,1,1,1,1],
//              [1,0,0,0,0,0,1],
//              [1,0,1,1,1,0,1],
//              [1,0,1,0,1,0,1],
//              [1,0,1,1,1,0,1],
//              [1,0,0,0,0,0,1],
//              [1,1,1,1,1,1,1]]
// 输出：2

// Exclude connected group of 0s on the corners because they are not closed island.
// Return number of connected component of 0s on the grid.
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut checked = vec![];
        let mut components = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if checked.contains(&(i, j)) {
                    continue;
                } else {
                    if grid[i][j] == 0 {
                        let mut component = std::collections::HashSet::new();
                        component.insert((i, j));
                        let mut stack = vec![(i,j)];
                        while !stack.is_empty() {
                            let (a,b) = stack.pop().unwrap();
                            if a >= 1 && grid[a-1][b] == 0 && !component.contains(&(a-1, b)) {
                                stack.push((a-1, b));
                                component.insert((a-1, b));
                            }
                            if a + 1 <= grid.len() - 1 && grid[a+1][b] == 0 && !component.contains(&(a+1, b)) {
                                stack.push((a+1, b));
                                component.insert((a+1, b));
                            }
                            if b + 1 <= grid[0].len() - 1 && grid[a][b+1] == 0 && !component.contains(&(a, b+1)) {
                                stack.push((a, b+1));
                                component.insert((a, b+1));
                            }
                            if b >= 1 && grid[a][b-1] == 0 && !component.contains(&(a, b-1)) {
                                stack.push((a, b-1));
                                component.insert((a, b-1));
                            }
                        }
                        components.push(component.clone());
                        for item in component {
                            checked.push(item);
                        }
                    }
                }
            }
        }
        for com in components {
            let mut flag = true;
            for (a,b) in com {
                if a == 0 || a == grid.len()-1 || b == 0 || b == grid[0].len()-1 {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
    
}

// 将checked从Vec更改为HashSet，以获得更快的查询性能。
// 使用递归函数简化深度优先搜索（DFS），避免显式堆栈操作。
// 使用一个函数来检查边界条件，使代码更简洁。
use std::collections::HashSet;

// 主函数，计算封闭岛屿的数量
pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut checked: HashSet<(usize, usize)> = HashSet::new(); // 存储已访问过的坐标
    let mut components = vec![]; // 存储每个连通分量（岛屿）的坐标集合

    // 遍历整个网格
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // 如果当前坐标未访问过且值为0（陆地）
            if !checked.contains(&(i, j)) && grid[i][j] == 0 {
                let mut component = HashSet::new(); // 存储当前连通分量（岛屿）的坐标
                dfs(&grid, i, j, &mut component, &mut checked); // 深度优先搜索遍历当前岛屿
                components.push(component); // 将当前岛屿添加到components中
            }
        }
    }

    // 遍历所有的连通分量（岛屿）
    for com in components {
        // 如果每个坐标都不在边界上，则计数增加
        if com.iter().all(|&(a, b)| !is_boundary(a, b, &grid)) {
            ans += 1;
        }
    }

    ans
}

// 深度优先搜索（DFS）函数
fn dfs(
    grid: &[Vec<i32>],
    i: usize,
    j: usize,
    component: &mut HashSet<(usize, usize)>,
    checked: &mut HashSet<(usize, usize)>
) {
    // 如果已经访问过该坐标或该坐标是水（值为1），则返回
    if !checked.insert((i, j)) || grid[i][j] == 1 {
        return;
    }
    component.insert((i, j)); // 将当前坐标添加到连通分量（岛屿）中

    // 构造当前坐标的四个邻居
    let neighbors = vec![(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)];

    // 遍历邻居，进行深度优先搜索
    for (a, b) in neighbors {
        if a < grid.len() && b < grid[0].len() {
            dfs(grid, a, b, component, checked);
        }
    }
}

// 辅助函数，检查是否在边界上
fn is_boundary(a: usize, b: usize, grid: &[Vec<i32>]) -> bool {
    a == 0 || a == grid.len() - 1 || b == 0 || b == grid[0].len() - 1
}


impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(grid.len(),grid[0].len());
        let mn=m*n;
        let mut parent:Vec<usize>=(0..=mn).collect();
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px]=py;
            }
        };
        for i in 0..m{
            for j in 0..n{
                if grid[i][j]==1{
                    continue
                }
                if i==0||i==m-1||j==0||j==n-1{
                    unite(i*n+j,mn,&mut parent);
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni<0||ni==m as i32||nj<0||nj==n as i32{
                        continue
                    }
                    let (ni,nj)=(ni as usize,nj as usize);
                    if grid[ni][nj]==0{
                        unite(i*n+j,ni*n+nj,&mut parent);
                    }
                }
            }
        }
        let pmn=find(mn,&mut parent);
        let mut ans=std::collections::HashSet::new();
        for i in 0..m{
            for j in 0..n{
                if grid[i][j]==0 {
                    let pij=find(i*n+j,&mut parent);
                    if pij!=pmn{
                        ans.insert(pij);
                    }
                }
            }
        }
        
        ans.len() as _
    }
}