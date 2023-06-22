// 2496_Maximum_Value_of_a_String_in_an_Array
// https://leetcode.cn/problems/maximum-value-of-a-string-in-an-array/

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut ans = i32::MIN;
        for s in strs {
            if s.chars().all(|x| x as u8 <= '9' as u8 && x as u8 >= '0' as u8) {
                ans = ans.max(s.parse::<i32>().unwrap());
            } else {
                ans = ans.max(s.len() as i32);
            }
        }
        ans
    }
}


fn parse(dat: &[u8]) -> i32 {
    let mut v = 0;
    for &b in dat.iter() {
        if b.is_ascii_digit() {
            v *= 10;
            v += (b - b'0') as i32;
        } else {
            return dat.len() as i32;
        }
    }
    v
}
impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter().map(|s| parse(s.as_bytes())).max().unwrap()
    }
}
