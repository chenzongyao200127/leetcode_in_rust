fn main() {
    assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
}


pub fn num_distinct(s: String, t: String) -> i32 {
    let s: Vec<_> = s.chars().collect();
    let t: Vec<_> = t.chars().collect();

    let mut dp = vec![vec![0; s.len()+1]; t.len()+1];

    for i in 0..s.len() {
        if s[i] == t[0] {
            dp[1][i+1] = dp[1][i] + 1;
        } else {
            dp[1][i+1] = dp[1][i];
        }
    }

    for i in 1..t.len() {
        for j in 0..s.len() {
            if s[j] == t[i] {
                dp[i+1][j+1] = dp[i+1][j] + dp[i][j];
            } else {
                dp[i+1][j+1] = dp[i+1][j]
            }
        }
    }

    println!("{:?}", dp);

    dp[t.len()][s.len()]
}




pub fn min_cut(s: String) -> i32 {
    let mut s1 = s.clone();
    let mut s2: String;
    let mut cnt = 0;

    loop {
        s2 = s1.chars().rev().collect();
        match longest_common_substring(&s1, &s2) {
            Some(common) => {
                s1 = remove_substring_from(&s1, common);
                cnt += 1;
            },
            None => break,
        }
    }

    cnt
}

pub fn longest_common_substring<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let mut longest_len = 0;
    let mut longest_pos = 0;

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1_bytes[i-1] == s2_bytes[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
                if dp[i][j] > longest_len {
                    longest_len = dp[i][j];
                    longest_pos = i - longest_len;
                }
            }
        }
    }

    if longest_len == 0 {
        None
    } else {
        Some(&s1[longest_pos..longest_pos + longest_len])
    }
}

pub fn remove_substring_from(s: &str, sub: &str) -> String {
    s.replace(sub, "")
}


use std::collections::HashMap;

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
        return false;
    }

    if desired_total <= max_choosable_integer {
        return true;
    }

    fn dfs(
        state: i32, 
        cur_total: i32, 
        max_choosable_integer: i32, 
        desired_total: i32, 
        memo: &mut HashMap<i32, bool>
    ) -> bool {
        if let Some(&v) = memo.get(&state) {
            return v;
        }

        for i in (1..=max_choosable_integer).rev() {
            if (state & (1 << (i - 1))) == 0 {
                if cur_total + i >= desired_total 
                    || !dfs(
                        state | (1 << (i - 1)), 
                        cur_total + i, 
                        max_choosable_integer, 
                        desired_total, 
                        memo
                    ) {
                    memo.insert(state, true);
                    return true;
                }
            }
        }

        memo.insert(state, false);
        return false;
    }

    let mut memo = HashMap::new();
    dfs(0, 0, max_choosable_integer, desired_total, &mut memo)
}
