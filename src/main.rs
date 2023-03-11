
pub fn main() {
    let ans = remove_kdigits("1432219".to_string(), 3);
    assert_eq!(ans, "1219".to_string());
}

pub fn remove_kdigits(num: String, k: i32) -> String {
    let len = num.len();
    let mut cnt = k;
    if len ==  k as usize {
        return "0".to_string()
    }
    let s: Vec<char> = num.chars().collect();
    let mut stk = vec![];
    for ch in s {
        while !stk.is_empty() && cnt > 0 && ((ch as i32) < (stk[stk.len()-1] as i32)) {
            stk.pop();
            cnt -= 1;
        }
        stk.push(ch);
    }
    unimplemented!();
    stk.iter().take(len-k as usize).collect::<String>()
}   