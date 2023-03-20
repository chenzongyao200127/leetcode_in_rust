pub fn main() {
    let ans = find_lex_smallest_string("5525".to_string(), 9, 2);
    assert_eq!(ans, 2050.to_string());
}

pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let n = s.len();
    let mut s = s.repeat(2);
    let mut vis = vec![0; n];
    let mut res = s; 
    // 将 s 延长一倍，方便截取轮转后的字符串 t
    let mut idx = 0;
    while vis[idx] == 0 {
        vis[idx] = 1;
        let k_limit = if b % 2 == 0 { 0 } else { 9 };
        // 每次进行累加之前，重新截取 t
        for k in 0..k_limit {
            
        }


        idx = (idx + b as usize) % n;
    }

    res
}