// 504. Base 7
// https://leetcode.cn/problems/base-7/

// 给定一个整数 num，将其转化为 7 进制，并以字符串形式输出。
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num < 0 { return format!("-{}", Solution::convert_to_base7(-num)); }
        if num < 7 { return format!("{}", num); }
        format!("{}{}", Solution::convert_to_base7(num / 7), Solution::convert_to_base7(num % 7))
    }
}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string()
        }
        let mut ans = "".to_string();
        let mut n = num.abs();
        while n != 0 {
            ans += &(n % 7).to_string();
            n /= 7;
        }
        if num.signum() < 0 {
            ans += "-";
        }
        ans.chars().rev().collect::<String>()
    }
}

// class Solution {
//     public String convertToBase7(int num) {
//         return Integer.toString(num, 7);
//     }
// }
// 作者：kyushu
// 链接：https://leetcode.cn/problems/base-7/solution/rustgojava-di-gui-100-by-kyushu-upm1/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。