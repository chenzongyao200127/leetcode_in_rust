// 405. Convert a Number to Hexadecimal
// https://leetcode.cn/problems/convert-a-number-to-hexadecimal/

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut ans = String::new();
        let mut num = num as u32;
        let s = "0123456789abcdef".chars().collect::<Vec<_>>();
        while num != 0 {
            ans = s[(num & 0xf) as usize].to_string() + &ans;
            num >>= 4;
        }
        ans
    }
}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut num = num as u32;
        let mut ans = String::new();
        let s = "0123456789abcdef".chars().collect::<Vec<_>>();
        while num != 0 {
            ans = s[(num & 0xf) as usize].to_string() + &ans;
            num >>= 4;
        }
        ans
    }
}
// 作者：tab-liu
// 链接：https://leetcode.cn/problems/convert-a-number-to-hexadecimal/solution/405-shu-zi-zhuan-huan-wei-shi-liu-jin-zh-6pj8/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{:x}", num)    
    }
}
