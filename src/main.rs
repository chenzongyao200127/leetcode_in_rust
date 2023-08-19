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
