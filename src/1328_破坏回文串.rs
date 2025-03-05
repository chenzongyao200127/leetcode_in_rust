// 1328_破坏回文串

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let s = palindrome.as_bytes();
        let n = s.len();
        if n == 1 {
            return "".to_string();
        }
        let mut s = s.to_vec();
        // if s[i] != 'a', we change it to 'a'
        // if s[i] == 'a', we change s[n - 1] to 'b'
        for i in 0..n / 2 {
            if s[i] != b'a' {
                s[i] = b'a';
                return String::from_utf8(s).unwrap();
            }
        }
        // currently, all s[i] == 'a'
        s[n - 1] = b'b';
        String::from_utf8(s).unwrap()
    }
}
