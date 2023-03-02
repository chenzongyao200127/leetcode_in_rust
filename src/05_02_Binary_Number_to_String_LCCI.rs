// 05.02. Binary Number to String LCCI
// https://leetcode.cn/problems/bianry-number-to-string-lcci/

/// Given a real number between 0 and 1 (e.g., 0.72) that is passed in as a double,
/// print the binary representation. 
/// If the number cannot be represented accurately in binary with at most 32 characters, print "ERROR".
/// 

/// 根据上述结论，将实数的十进制表示转换成二进制表示的方法是：每次将实数乘以2，将此时的整数部分添加到二进制表示的末尾，然后将整数部分置为 0，
/// 重复上述操作，直到小数部分变成 0 或者小数部分出现循环时结束操作。
/// 当小数部分变成 0 时，得到二进制表示下的有限小数；
/// 当小数部分出现循环时，得到二进制表示下的无限循环小数
impl Solution {
    pub fn print_bin(num: f64) -> String {
        let mut ans = "0.".to_string();
        let mut dig = -1;
        let mut n = num;
        let base = 2.0 as f64;
        while n != 0.0 {
            if n >= base.powi(dig) {
                n = n - base.powi(dig);
                ans.push('1');
            } else {
                ans.push('0');
            }
            dig -= 1;
            if dig == -30 {
                break;
            }
        }
    
        if n == 0.0 {
            ans
        } else {
            "ERROR".to_string()
        }
    }
}

impl Solution {
    pub fn print_bin(num: f64) -> String {
        let (mut x, mut ans) = (num, "0.".to_string());
        for _ in 0..32 {
            ans.push(if x * 2.0 < 1.0 {'0'} else {'1'});
            x = x * 2.0 - ((x * 2.0) as i32) as f64;
            if x == 0.0 { break; }
        }
        if ans.len() < 32 { ans } else { "ERROR".to_string() }
    }
}

// class Solution:
//     def printBin(self, num: float) -> str:
//         res = "0."
//         while len(res) <= 32 and num != 0:
//             num *= 2
//             digit = int(num)
//             res += str(digit)
//             num -= digit
//         return res if len(res) <= 32 else "ERROR"


// class Solution {
//     public:
//         string printBin(double num) {
//             double a[10] = {0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125};
//             string ans = "0.";
//             int idx = 0;
//             for(int i = 0; i < 7; i++) {
//                 if(num >= a[i]) {
//                     num -= a[i];
//                     ans += '1';
//                 }
//                 else if(num != .0) ans += '0';
//                 else break;
//                 // cout << num << endl;
//             }
//             if(num != 0.0) return "ERROR";
//             return ans;
//         }
//     };