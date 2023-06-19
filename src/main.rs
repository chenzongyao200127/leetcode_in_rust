use std::collections::HashSet;

fn num_permutations(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();

    fn backtrack(index: usize, nums: &[i32], used: &mut HashSet<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
        if index == nums.len() {
            return 1;
        }

        if let Some(count) = memo[index] {
            return count;
        }

        let mut count = 0;
        for i in 0..nums.len() {
            if !used.contains(&nums[i]) && (index == 0 || nums[index - 1] % nums[i] == 0 || nums[i] % nums[index - 1] == 0) {
                used.insert(nums[i]);
                count += backtrack(index + 1, nums, used, memo);
                count %= MOD;
                used.remove(&nums[i]);
            }
        }
        memo[index] = Some(count);
        count
    }

    let mut used = HashSet::new();
    let mut memo = vec![None; n];
    backtrack(0, &nums, &mut used, &mut memo)
}


pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut components = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !checked.contains(&(i, j)) && grid[i][j] == 0 {
                let mut component = HashSet::new();
                dfs(&grid, i, j, &mut component, &mut checked);
                components.push(component);
            }
        }
    }

    for com in components {
        if com.iter().all(|&(a, b)| !is_boundary(a, b, &grid)) {
            ans += 1;
        }
    }

    ans
}

fn dfs(
    grid: &[Vec<i32>],
    i: usize,
    j: usize,
    component: &mut HashSet<(usize, usize)>,
    checked: &mut HashSet<(usize, usize)>
) {
    if !checked.insert((i, j)) || grid[i][j] == 1 {
        return;
    }
    component.insert((i, j));

    let neighbors = vec![(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)];

    for (a, b) in neighbors {
        if a < grid.len() && b < grid[0].len() {
            dfs(grid, a, b, component, checked);
        }
    }
}

fn is_boundary(a: usize, b: usize, grid: &[Vec<i32>]) -> bool {
    a == 0 || a == grid.len() - 1 || b == 0 || b == grid[0].len() - 1
}




pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0, i32::MIN, i32::MIN];
    
    for num in nums {
        let mut temp_dp = dp.clone();
        for i in 0..3 {
            let cmod = ((dp[i] + num) % 3 + 3) % 3;
            temp_dp[cmod as usize] = temp_dp[cmod as usize].max(dp[i] + num);
        }
        dp = temp_dp;
    }
    dp[0]
}



fn main() {
    // let grid = vec![vec![1,1,1,1,1,1,1],
    //                                 vec![1,0,0,0,0,0,1],
    //                                 vec![1,0,1,1,1,0,1],
    //                                 vec![1,0,1,0,1,0,1],
    //                                 vec![1,0,1,1,1,0,1],
    //                                 vec![1,0,0,0,0,0,1],
    //                                 vec![1,1,1,1,1,1,1]];
    // let grid = vec![vec![1,1,1,1,1,1,1,0],vec![1,0,0,0,0,1,1,0],vec![1,0,1,0,1,1,1,0],vec![1,0,0,0,0,1,0,1],vec![1,1,1,1,1,1,1,0]];
    // println!("{:?}", closed_island(grid));

    println!("{:?}", max_sum_div_three(vec!
        [1,2,3,4,4]));

    println!("{:?}", max_sum_div_three(vec!
            [4]));
}


// fn num_permutations(nums: Vec<i32>) -> i32 {
//     const MOD: i32 = 1_000_000_007;
//     let n = nums.len();

//     fn backtrack(index: usize, path: &mut Vec<i32>, nums: &[i32], count: &mut i32) {
//         if index == nums.len() {
//             *count += 1;
//             *count %= MOD;
//             return;
//         }

//         for i in 0..nums.len() {
//             if !path.contains(&nums[i]) && (index == 0 || path[index - 1] % nums[i] == 0 || nums[i] % path[index - 1] == 0) {
//                 path.push(nums[i]);
//                 backtrack(index + 1, path, nums, count);
//                 path.pop();
//             }
//         }
//     }

//     let mut count = 0;
//     let mut path = vec![];
//     backtrack(0, &mut path, &nums, &mut count);
//     count
// }