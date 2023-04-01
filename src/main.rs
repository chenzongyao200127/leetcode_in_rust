pub fn main() {
    let ans = mask_pii("LeetCode@LeetCode.com".to_string());
    assert_eq!("l*****e@leetcode.com".to_string(), ans);
}


pub fn mask_pii(s: String) -> String {
    if let Some(idx) = s.find('@') {
        let s = s.to_ascii_lowercase();
        let mut new_s = s[0..1].to_string();
        new_s.push_str(&"*****");
        new_s.push_str(&s[idx-1..s.len()]);
        return new_s
    } else {
        let splits = vec!['+', '-', '(', ')', ' '];
        let s: Vec<char> = s.chars().collect();
        let mut new_s = vec![];
        s.iter().for_each(|ch| {
            if !splits.contains(ch) {
                new_s.push(*ch)
            }
        });
        let mut ans = "".to_string();
        match new_s.len() {
            10 => ans.push_str(&"***-***-"),
            11 => ans.push_str(&"+*-***-***-"),
            12 => ans.push_str(&"+**-***-***-"),
            _  => ans.push_str(&"+**-***-***-"),
        }
        for i in new_s.len()-4..new_s.len() {
            ans.push(new_s[i]);
        }
        return ans;
    }
}
