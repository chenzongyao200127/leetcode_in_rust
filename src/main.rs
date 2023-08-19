fn main() {
}


pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1_v: Vec<char> = s1.chars().collect();
    let s2_v: Vec<char> = s2.chars().collect();
    let s3_v: Vec<char> = s3.chars().collect();

    let mut x = 0;
    let mut y = 0;
    for i in 0..s3_v.len() {
        if x < s1_v.len() && s3_v[i] == s1_v[x] && s3_v[i] != s2_v[y] {
            x += 1
        } else if y < s2_v.len() && s3_v[i] == s2_v[y] && s3_v[i] != s1_v[x] {
            y += 1
        } else if s3_v[i] == s1_v[x] && s3_v[i] == s2_v[y]{
            is_interleave(s1[x+1..].to_string(), s2[y..].to_string(), s3[i+1..].to_string());
            is_interleave(s1[x..].to_string(), s2[y+1..].to_string(), s3[i+1..].to_string());
        } else {
            return false
        }
    }

    true
}