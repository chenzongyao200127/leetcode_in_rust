// 402_Remove_K_Digits
// https://leetcode.cn/problems/remove-k-digits/

impl Solution {
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
        
        unimplemented!(); //未完成：处理stk中的前导0
        
        stk.iter().take(len-k as usize).collect::<String>()
    }   
}



impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let chars = num.chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut dequeue: Vec<char> = Vec::new();
        dequeue.push(chars[0]);
        for i in 1..chars.len() {
            let c = chars[i];
            if count >= k {
                dequeue.push(c);
                continue;
            }
            loop {
                if (!dequeue.is_empty()) && count < k && c < *dequeue.last().unwrap() {
                    dequeue.pop();
                    count += 1;
                } else {
                    dequeue.push(c);
                    break;
                }
            }
        }
        let mut ans = String::from("");
        let mut maybe_first_0 = true;
        for i in 0..(dequeue.len() - (k - count) as usize) {
            let c = dequeue[i];
            if maybe_first_0 && c == '0' {
                continue;
            } else {
                ans.push(dequeue[i]);
                maybe_first_0 = false;
            }
        }
        if ans == "".to_string() {
            ans = "0".to_string();
        }
        return ans;
    }
}
