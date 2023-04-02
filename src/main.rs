fn main() {
    let s1 = "01000111";
    let s2 = "00111";
    let s3 = "111";

    println!("Example 1: {}", find_the_longest_balanced_substring(s1.to_string())); // Output: 6
    println!("Example 2: {}", find_the_longest_balanced_substring(s2.to_string())); // Output: 4
    println!("Example 3: {}", find_the_longest_balanced_substring(s3.to_string())); // Output: 0
}

pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let mut s: Vec<char> = s.chars().collect();
    s.insert(0, '1');
    s.push('0');
    let mut idx = 0;
    let mut collect = vec![];
    for i in 0..s.len()-1 {
        if s[i] == '1' && s[i+1] == '0' {
            let mut tmp = vec![];
            for j in idx..=i {
                tmp.push(s[j]);
            }
            idx = i+1;
            // println!("{:?}", tmp);
            collect.push(tmp);
        }
    }
    let mut ans = 0;
    for tmp in collect {
        let cnt_0 = tmp.iter().filter(|&&x| x == '0').count();
        let cnt_1 = tmp.iter().filter(|&&x| x == '1').count();
        let cnt = cnt_0.min(cnt_1);
        ans = ans.max(cnt * 2);
    }

    ans as i32
}