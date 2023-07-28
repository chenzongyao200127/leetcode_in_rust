use std::collections::HashMap;
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

pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut dp = vec![vec![vec![0; n as usize]; n as usize]; k as usize + 1];
    dp[0][row as usize][column as usize] = 1;
    let mut out_counts = 0;
    let mut stay_counts = 0;

    for i in 0..k {
        for j in 0..n {
            for k in 0..n {
                if dp[i as usize][j as usize][k as usize] > 0 {
                    for &(dx, dy) in [(-2, -1), (-1, -2), (1, 2), (2, 1), 
                                      (-2, 1), (-1, 2), (1, -2), (2, -1)].iter() {
                        let new_x = j + dx;
                        let new_y = k + dy;
                        if new_x >= 0 && new_x < n && new_y >= 0 && new_y < n {
                            dp[i as usize + 1][new_x as usize][new_y as usize] = 
                                dp[i as usize + 1][new_x as usize][new_y as usize] 
                                    + dp[i as usize][j as usize][k as usize];
                        } else {
                            out_counts = out_counts + dp[i as usize][j as usize][k as usize];
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            stay_counts += dp[k as usize][i as usize][j as usize];
        }
    }

    return (out_counts) as f64 / (out_counts + stay_counts) as f64
}

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mut dp = vec![vec![vec![0; n as usize]; m as usize]; max_move as usize + 1];
    dp[0][start_row as usize][start_column as usize] = 1;
    let mut out_counts = 0;
    let mod_num = 1_000_000_007;

    for i in 0..max_move {
        for j in 0..m {
            for k in 0..n {
                if dp[i as usize][j as usize][k as usize] > 0 {
                    for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                        let new_x = j + dx;
                        let new_y = k + dy;
                        if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n {
                            dp[i as usize + 1][new_x as usize][new_y as usize] = 
                                (dp[i as usize + 1][new_x as usize][new_y as usize] 
                                    + dp[i as usize][j as usize][k as usize]) % mod_num;
                        } else {
                            out_counts = (out_counts + dp[i as usize][j as usize][k as usize]) % mod_num;
                        }
                    }
                }
            }
        }
    }
    out_counts
}

fn main() {
    let ans = minimum_time(3, vec![vec![1,3],vec![2,3]], vec![3,2,5]);
    assert_eq!(ans, 4);
}

