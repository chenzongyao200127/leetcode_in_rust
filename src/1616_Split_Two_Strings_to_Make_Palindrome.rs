// 1616_Split_Two_Strings_to_Make_Palindrome
// https://leetcode.cn/problems/split-two-strings-to-make-palindrome/

/// 写了一个半小时，Debug好几次，终于过了...
// 1616_Split_Two_Strings_to_Make_Palindrome
// https://leetcode.cn/problems/split-two-strings-to-make-palindrome/

/// 写了一个半小时，Debug好几次，终于过了... 时间效率很低
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        if a == a.chars().rev().collect::<String>() || b == b.chars().rev().collect::<String>() {
            return true;
        }

        let len = a.len();
        let (mut i, mut j) = (0, 0);

        let mut initi = 0;
        while i < len {
            if a[i..i+1] == b[b.len()-1-i..b.len()-i] {
                i += 1;
            } else {
                initi = i;
                break;
            }
        }
        if i >= len / 2 {
            return true;
        }

        let mut ii = initi;
        let mut ij = initi;
        while ii < len {
            if b[ii..ii+1] == b[a.len()-1-ii..a.len()-ii] {
                ii += 1;
            } else {
                break;
            }
        }
        while ij < len {
            if a[ij..ij+1] == a[a.len()-1-ij..a.len()-ij] {
                ij += 1;
            } else {
                break;
            }
        }
        if ij >= len/2 || ii >= len/2 {
            return true;
        }

        // println!("{:?}", (ii, ij));
    
        let mut initj = 0;
        while j < len {    
            if b[j..j+1] == a[a.len()-1-j..a.len()-j] {
                j += 1;
            } else {
                initj = j;
                break;
            }
        }
        if j >= len / 2 {
            return true;
        }

        let mut ji = initj;
        let mut jj = initj;
        while ji < len {
            if a[ji..ji+1] == a[a.len()-1-ji..a.len()-ji] {
                ji += 1;
            } else {
                break;
            }
        }
        while jj < len {
            if b[jj..jj+1] == b[a.len()-1-jj..a.len()-jj] {
                jj += 1;
            } else {
                break;
            }
        }
        if jj >= len/2 || ji >= len/2 {
            return true;
        }

        false
    }
}

/// 示例代码：双指针
/// 和俺思路一致（分4种情况讨论），但是官解写得优雅许多
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        fn is_palindrome(a: &[u8], mut left: usize, mut right: usize) -> bool {
            while left <= right {
                if a[left] != a[right] {
                    return false;
                }

                left += 1;
                right -= 1;
            }

            return true;
        }

        fn check(a: &[u8], b: &[u8]) -> bool {
            let (mut left, mut right) = (0, b.len() - 1);

            while left < right {
                if a[left] != b[right] {
                    break;
                }
                left +=1;
                right -= 1;
            }

            if left >= right {
                return true;
            }

            is_palindrome(a, left, right) || is_palindrome(b, left, right)
        }

        let a = a.as_bytes();
        let b = b.as_bytes();

        check(a, b) || check(b, a)
    }
}


/// 中心扩展
/// 题目给出的字符串长度固定，可以直接使用 中心扩展法 检测
// class Solution {
//     public:
//         bool checkPalindromeFormation(string a, string b) {
//             int left = a.size() / 2 - 1;
//             left = min(check(a, a, left), check(b, b, left));
//             left = min(check(a, b, left), check(b, a, left));
//             return left == -1;
//         }
    
//         int check(string str_l, string str_r, int left) {
//             int right = str_l.size() - 1 - left;
//             while (left >= 0 && right < str_l.size()) {
//                 if (str_l[left] != str_r[right]) break;
//                 left--;
//                 right++;
//             }
//             return left;
//         }
//     };
    
//     作者：ikaruga
//     链接：https://leetcode.cn/problems/split-two-strings-to-make-palindrome/solution/split-two-strings-to-make-palindrome-by-ikaruga/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

// class Solution {
//     public boolean checkPalindromeFormation(String a, String b) {
//         return f(a,b)||f(b,a);
//     }
//     boolean f(String a, String b) {
//         int m=a.length(),i=0,j=m-1;
//         for(;i<m&&a.charAt(i)==b.charAt(j);i++,j--);
//         for(;i<j&&b.charAt(i)==b.charAt(j);i++,j--);
//         for(;i<j&&a.charAt(i)==a.charAt(j);i++,j--);
//         return i>=j;
//     }
// }

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        fn check(a: &[u8], b: &[u8], mut left: i32) -> i32 {
            let mut right = a.len() - 1 - left as usize;
            while left >= 0 && right < a.len() {
                if a[left as usize] != b[right] {
                    break;
                }
                left -= 1;
                right += 1;
            }
            
            return left
        }

        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut left = (a.len() / 2 - 1) as i32;
        left = check(a, a, left as i32).min(check(b, b, left as i32));
        left = check(a, b, left as i32 ).min(check(b, a, left as i32));

        left == -1
    }
}