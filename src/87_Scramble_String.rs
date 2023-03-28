// 87_Scramble_String
// https://leetcode.cn/problems/scramble-string/


/// 超时
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
         
        is_scramble(&s1, &s2)
    }
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


struct Solution {
    cache: Vec<Vec<Vec<i32>>>,
    s1: String,
    s2: String,
    n: usize,
    no: i32,
    yes: i32,
    empty: i32,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            cache: Vec::new(),
            s1: String::new(),
            s2: String::new(),
            n: 0,
            no: -1,
            yes: 1,
            empty: 0,
        }
    }

    pub fn is_scramble(&mut self, s1: String, s2: String) -> bool {
        self.s1 = s1;
        self.s2 = s2;
        if self.s1 == self.s2 {
            return true;
        }
        if self.s1.len() != self.s2.len() {
            return false;
        }
        self.n = self.s1.len();
        self.cache = vec![vec![vec![0; self.n + 1]; self.n]; self.n];

        self.dfs(0, 0, self.n)
    }

    fn dfs(&mut self, i: usize, j: usize, len: usize) -> bool {
        if self.cache[i][j][len] != self.empty {
            return self.cache[i][j][len] == self.yes;
        }

        let a = &self.s1[i..i + len];
        let b = &self.s2[j..j + len];

        if a == b {
            self.cache[i][j][len] = self.yes;
            return true;
        }

        if !self.check(a, b) {
            self.cache[i][j][len] = self.no;
            return false;
        }

        for k in 1..len {
            if self.dfs(i, j, k) && self.dfs(i + k, j + k, len - k) {
                self.cache[i][j][len] = self.yes;
                return true;
            }

            if self.dfs(i, j + len - k, k) && self.dfs(i + k, j, len - k) {
                self.cache[i][j][len] = self.yes;
                return true;
            }
        }

        self.cache[i][j][len] = self.no;
        false
    }

    fn check(&self, a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut cnt1 = vec![0; 26];
        let mut cnt2 = vec![0; 26];
        for c in a.chars() {
            cnt1[(c as u8 - b'a') as usize] += 1;
        }
        for c in b.chars() {
            cnt2[(c as u8 - b'a') as usize] += 1;
        }
        cnt1 == cnt2
    }
}



// 87
pub fn count_same(s1: &[u8], s2: &[u8]) -> u8 {
    let mut cnt = vec![0; 26];
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            for c in s1.iter() {
                cnt[(*c - 'a' as u8) as usize] += 1;
            }
            for c in s2.iter() {
                cnt[(*c - 'a' as u8) as usize] -= 1;
            }
            for i in cnt.iter() {
                if *i != 0 {
                    return 0;
                }
            }
            return 2;
        }
    }
    1
}

pub fn p87(s1: &[u8], s2: &[u8], i1: usize, i2: usize, l: usize, f: &mut Vec<Vec<Vec<u8>>>) -> u8 {
    if f[i1][i2][l] != 2 {
        return f[i1][i2][l];
    }
    f[i1][i2][l] = count_same(&s1[i1..(i1+l)], &s2[i2..(i2+l)]);
    if f[i1][i2][l] != 2 {
        return f[i1][i2][l];
    }

    for k in 1..l {
        let p1 = p87(s1, s2, i1, i2, k, f) & p87(s1, s2, i1 + k, i2 + k, l - k, f);
        if p1 == 1 {
            f[i1][i2][l] = 1;
            return 1;
        }
        let p2 = p87(s1, s2, i1, i2 + l - k, k, f) & p87(s1, s2, i1 + k, i2, l - k, f);
        if p2 == 1 {
            f[i1][i2][l] = 1;
            return 1;
        }
    }
    f[i1][i2][l] = 0;
    0
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut f = vec![vec![vec![2; s1.len() + 1]; s1.len() + 1]; s1.len() + 1];
        p87(s1, s2, 0, 0, s1.len(), &mut f) == 1
    }
}