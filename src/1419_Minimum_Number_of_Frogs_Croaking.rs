// 1419_Minimum_Number_of_Frogs_Croaking
// https://leetcode.cn/problems/minimum-number-of-frogs-croaking/


impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let (mut c, mut r, mut o, mut a, mut k, mut curr, mut ret) = (0, 0, 0, 0, 0, 0, 0);
        for ch in croak_of_frogs.bytes() {
            if ch == b'c' {
                c += 1;
                curr += 1;
            }
            if ch == b'r' { r += 1; }
            if ch == b'o' { o += 1; }
            if ch == b'a' { a += 1; }
            if ch == b'k' {
                k += 1;
                curr -= 1;
            }
            ret = ret.max(curr);
            if c < r || r < o || o < a || a < k { return -1; }
        }
        if curr == 0 && c == r && r == o && o == a && a == k { ret } else { -1 }
    }
}



impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut croak = [0; 5];
        let mut result = 0;
        for &ch in croak_of_frogs.as_bytes() {
            let idx = match ch {
                b'c' => 0,
                b'r' => 1,
                b'o' => 2,
                b'a' => 3,
                b'k' => 4,
                _ => { return -1; }
            };
            if idx == 0 {
                if croak[4] > 0 {
                    croak[4] -= 1;
                } else {
                    result += 1;
                }
            } else {
                croak[idx - 1] -= 1;
            }
            croak[idx] += 1;
            if croak.iter().any(|x| *x < 0) { return -1; }
        }
        if croak[..4].iter().all(|x| *x == 0) {
            return result;
        }
        -1
    }
}