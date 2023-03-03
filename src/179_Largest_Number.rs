// 179. Largest Number
// https://leetcode.cn/problems/largest-number/

// 自定义一种排序方式 比较 s1 + s2 和 s2 + s1
use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        let cmp = |x, y| -> Ordering {
            let (x, y) = (x as i64, y as i64);
            let (mut sx, mut sy) = (10, 10);
            while sx <= x {
                sx *= 10;
            }
            while sy <= y {
                sy *= 10;
            }
            (sx * y + x).cmp(&(sy * x + y))
        };

        nums.sort_unstable_by(|&a, &b| cmp(a, b));
        if nums[0] == 0 {
            return "0".to_string();
        }
        nums.iter().map(|i| i.to_string()).collect()
    }
}


// class Solution {
//     public String largestNumber(int[] nums) {
//         int n = nums.length;
//         String[] ss = new String[n];
//         for (int i = 0; i < n; i++) ss[i] = "" + nums[i];
//         Arrays.sort(ss, (a, b) -> {
//             String sa = a + b, sb = b + a ;
//             return sb.compareTo(sa);
//         });
        
//         StringBuilder sb = new StringBuilder();
//         for (String s : ss) sb.append(s);
//         int len = sb.length();
//         int k = 0;
//         while (k < len - 1 && sb.charAt(k) == '0') k++;
//         return sb.substring(k);
//     }
// }

// 作者：AC_OIer
// 链接：https://leetcode.cn/problems/largest-number/solution/gong-shui-san-xie-noxiang-xin-ke-xue-xi-vn86e/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。




use std::cmp::Ordering::*;

impl Solution {
    fn token_cmp(a: &str, b: &str) -> std::cmp::Ordering {
        // dbg!(&a);
        // dbg!(&b);

        let a_len = a.len();
        let b_len = b.len();
        let min_len = a_len.min(b_len);
        let mut ret =
            a.chars().zip(b.chars()).fold(
                Equal,
                |acc, (c1, c2)| {
                    if acc == Equal {
                        c1.cmp(&c2)
                    } else {
                        acc
                    }
                },
            );
        if ret == Equal && a_len != b_len {
            let longer = if a_len > b_len { a } else { b };
            let diff = &longer[min_len..];
            let common = &longer[0..min_len];
            if longer == a {
                ret = Solution::token_cmp(diff, common)
            } else {
                ret = Solution::token_cmp(common, diff)
            }
        }

        // dbg!(&ret);
        ret
    }

    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut tokens = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        tokens.sort_unstable_by(|a, b| Solution::token_cmp(a, b));
        tokens.reverse();
        let mut ret = String::new();
        for ele in tokens {
            ret.push_str(&ele);
        }

        // check redundant zero
        let mut zero_cnt = 0;
        while ret.chars().nth(zero_cnt) == Some('0') {
            zero_cnt += 1;
        }
        if zero_cnt > 0 {
            ret = ret[zero_cnt..].to_string();
        }
        if ret.is_empty() {
            ret.push('0') 
        }
        ret
    }
}