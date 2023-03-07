pub fn main() {
    let ans = ways_to_reach_target(6, vec![vec![6,1],vec![3,2],vec![2,3]]);
    assert_eq!(ans, 7);
}

pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
    let _MOD = 1000_000_007;
    let type_len = types.len();

    let mut dp = vec![vec![0; target as usize + 1]; type_len + 1];
    dp[0][0] = 1; //dp[i][j] 前i题获得j分数所有的方法数
    for i in 1..=type_len {
        let tmp_score = types[i-1][1];
        for j in 0..=target as usize {
            let mut k = 0;
            while k <= types[i-1][0] && (j as i32 - tmp_score * k >= 0) {
                dp[i][j] += dp[i-1][j-(k*tmp_score) as usize];
                dp[i][j] = dp[i][j] % _MOD;
                k += 1;
            }
        }
    }

    dp[type_len][target as usize]
}