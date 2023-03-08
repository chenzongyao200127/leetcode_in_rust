pub fn main() {
    let ans = max_value(vec![vec![1,2,5],vec![3,2,1]]);
    assert_eq!(ans, 12);
}

pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid[0].len();
    let n = grid.len();
    let mut dp = vec![vec![0; m]; n];
    let mut pre_sum = 0;
    for i in 0..m {
        pre_sum += grid[0][i];
        dp[0][i] = pre_sum;
    }
    pre_sum = 0;
    for i in 0..n {
        pre_sum += grid[i][0];
        dp[i][0] = pre_sum;
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = dp[i][j-1].max(dp[i-1][j]) + grid[i][j];
            println!("{:?}", dp);
        }
    }
    dp[n-1][m-1]
}

// pub fn main() {
//     let ans = can_complete_circuit(vec![3,3,4], vec![3,4,4]);
//     assert_eq!(ans, -1);
// }

// pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//     let mut diff = vec![];
//     for i in 0..gas.len() {
//         diff.push(gas[i] - cost[i]);
//     }
//     let mut diff: Vec<_> = diff.iter().enumerate().collect();
//     diff.sort_unstable_by(|a, b| (b.1).cmp(&(a.1)));
//     let start = diff[0].0;
//     let mut fuel = gas[start];
//     let mut cur = start;
//     println!("{:?}", fuel);
//     while cur < gas.len()-1 {
//         fuel  -= cost[start];
//         if fuel < 0 {
//             return -1;
//         }
//         println!("{:?}", fuel);
//         cur += 1;
//         fuel += gas[cur];
//         println!("{:?}", fuel);
//     }
//     cur = 0;
//     while cur <= start {
//         if cur == 0 {
//             fuel -= cost[gas.len()-1];
//         } else {
//             fuel -= cost[cur - 1];
//         }
//         println!("{:?}", fuel);
//         if fuel < 0 {
//             return -1;
//         }
//         if cur == start {
//             break;
//         }
//         fuel += gas[cur];
//         println!("{:?}", fuel);
//         cur += 1;
//     }

//     start as i32
// }