pub fn main() {
    let ans = find_length_of_shortest_subarray(vec![5,4,3,2,1]);
    assert_eq!(ans, 4);

    let ans = find_length_of_shortest_subarray(vec![1,2,3]);
    assert_eq!(ans, 0);

    let ans = find_length_of_shortest_subarray(vec![1,2,3,10,0,7,8,9]);
    assert_eq!(ans, 2);

    let ans = find_length_of_shortest_subarray(vec![13,0,14,7,18,18,18,16,8,15,20]);
    assert_eq!(ans, 8);
}


pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut i = 0;
    let mut j = n - 1;

    while i < n - 1 && arr[i] <= arr[i + 1] {
        i += 1;
    }
    while j > 0 && arr[j-1] <= arr[j] {
        j -= 1;
    }
    if i >= j {
        return 0;
    }

    let mut ans = j.min(n-i-1);
    let mut right = j;
    for left in 0..i+1 {
        while right < n && arr[right] < arr[left] {
            right += 1;
        }
        ans = ans.min(right - left - 1);
    }
    
    ans as i32
}




    pub fn is_scramble(s1: &str, s2: &str) -> bool {
        if s1 == s2 {
            return true;
        }
        if !check(&s1, &s2) {
            return false;
        }
        for i in 1..s1.len() {
            let a = &s1[0..i];
            let b = &s1[i..s1.len()];
            let c = &s2[0..i];
            let d = &s2[i..s2.len()];
            if is_scramble(a, c) && is_scramble(b, d) {
                return true;
            }
            let e = &s2[0..s2.len()-i];
            let f = &s2[s2.len()-i..s2.len()];
            if is_scramble(a, f) && is_scramble(b, e) {
                return true;
            }
        }
        false
    }


pub fn check(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut cnt1 = vec![0; 26];
    let mut cnt2 = vec![0; 26];
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    for i in 0..s1.len() {
        cnt1[(s1[i] as u8 - 'a' as u8) as usize] += 1;
        cnt2[(s2[i] as u8 - 'a' as u8) as usize] += 1;
    }
    if cnt1 == cnt2 {
        return true;
    }

    false
}


pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 0..s1.len()+1 {
        dp[i][0] = 0;
    }

    for j in 0..s2.len()+1 {
        dp[0][j] = 0;
    }

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i] == s2[j] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }

    let mut sub_s = String::new();
    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 && j > 0 {
        if s1[i - 1] == s2[j - 1] {
            sub_s.push(s1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i-1][j] > dp[i][j-1] {
            sub_s.push(s1[i - 1]);
            i -= 1;
        } else {
            sub_s.push(s1[j - 1]);
            j -= 1;
        }
    }

    sub_s.chars().rev().collect()
}