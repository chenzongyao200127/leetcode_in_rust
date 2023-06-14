
pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let mut s = String::new();
    let mut ans = 0;
    for _ in 0..flips.len() {
        s.push('0');
    }
    let mut s = s.chars().collect::<Vec<char>>();
    for i in 0..flips.len() { 
        s[flips[i] as usize - 1] = '1';
        if is_prefix_aligned(&s, i+1) {
            ans += 1;
        }
    }
    ans
}

pub fn is_prefix_aligned(s: &Vec<char>, idx: usize) -> bool {
    let mut s_tmp = String::new();
    for _ in 0..idx { 
        s_tmp.push('1');
    }
    for _ in idx..s.len() {
        s_tmp.push('0');
    }
    &s_tmp.chars().collect::<Vec<char>>() == s
}

fn main() {
    println!("{:?}", num_times_all_blue(vec![3,2,4,1,5]));
}
