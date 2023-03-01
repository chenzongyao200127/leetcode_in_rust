// 564. Find the Closest Palindrome
// https://leetcode.cn/problems/find-the-closest-palindrome/

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let num = n.parse::<i64>().unwrap();
        if num == 11 as i64 {
            return "9".to_string();
        }
    
        let mut ans1 = num;
        let mut ans2 = num;
        let (mut i,mut j) = (0,0);
        let base: i64 = 10;
        while ans1 >= num {
            ans1 = get_pld(num - base.pow(i));
            i += 1;
        }
        while ans2 <= num {
            ans2 = get_pld(num + base.pow(j));
            j += 1;
        }
    
        if num-ans1 <= ans2 - num {
            return ans1.to_string();
        } else {
            return ans2.to_string();
        }
    }
}
    
    
    pub fn get_pld(num: i64) -> i64 {
        let s = num.to_string();
        let mut s_char: Vec<char> = s.chars().collect();
        for i in 0..(s.len())/2 {
            s_char[s.len()-1-i] = s_char[i];
        }
    
        let new_s =  s_char.iter().collect::<String>();
        new_s.parse::<i64>().unwrap()
    }



    impl Solution {
        fn rev(n: i64) -> i64 {
            let mut t = n;
            let mut ret = 0;
            while t > 0 {
                ret = ret * 10 + t % 10;
                t /= 10;
            }
            ret
        }
        fn split(n: i64) -> (i64, i64, i64, i64) {
            let s = n.to_string();
            let l = s.len();
            let a = s[0..l/2].to_string().parse::<i64>().unwrap_or(-1);
            let m = s[l/2..(l+1)/2].to_string().parse::<i64>().unwrap_or(-1);
            let b = s[(l+1)/2..].to_string().parse::<i64>().unwrap_or(-1);
            let f = 10_i64.pow(l as u32/2);
            //println!("a = {a}, m = {m}, b = {b}, f = {f}");
            (a, m, b, f)
        }
        fn search_dn(n: i64) -> i64 {
            let (a, m, b, f) = Self::split(n);
            if a == -1 {return 1;}
            let ra = Self::rev(a);
            if ra == b {return Self::search_dn(n - 1) + 1;}
            if ra < b {return b - ra;}
            if m > 0 {return b + f - ra;}
            if ra == 1 {return 1;}
            b + f - Self::rev(a - 1)
        }
        fn search_up(n: i64) -> i64 {
            let (a, m, b, f) = Self::split(n);
            if a == -1 {return 1;}
            let ra = Self::rev(a);
            if ra == b {return Self::search_up(n + 1) + 1;}
            if ra > b {return ra - b;}
            if m < 9 && m != -1 {return f - b + ra;}
            f - b + Self::rev(a + 1)
        }
        pub fn nearest_palindromic(n: String) -> String {
            let val = n.parse::<i64>().unwrap();
            let dn = Self::search_dn(val);
            let up = Self::search_up(val);
            if dn <= up {
                (val - dn).to_string()
            } else {
                (val + up).to_string()
            }
        }
    }
    