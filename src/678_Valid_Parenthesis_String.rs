// 678. Valid Parenthesis String
// https://leetcode.cn/problems/valid-parenthesis-string/

// 栈
/// 括号匹配的问题可以用栈求解。
/// 如果字符串中没有星号，则只需要一个栈存储左括号，在从左到右遍历字符串的过程中检查括号是否匹配。
/// 在有星号的情况下，需要两个栈分别存储左括号和星号。从左到右遍历字符串，进行如下操作。
/// - 如果遇到左括号，则将当前下标存入左括号栈。
/// - 如果遇到星号，则将当前下标存入星号栈。
/// - 如果遇到右括号，则需要有一个左括号或星号和右括号匹配，由于星号也可以看成右括号或者空字符串，
///   因此当前的右括号应优先和左括号匹配，没有左括号时和星号匹配：
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let s_chars: Vec<_> = s.chars().enumerate().collect();
        let mut left_bracket_stack = vec![];
        let mut asterisk_stack = vec![];
        for i in s_chars {
            match i.1 {
                '(' => {
                    left_bracket_stack.push(i);
                },
                '*' => {
                    asterisk_stack.push(i);
                }
                _ => {
                    if left_bracket_stack.is_empty() && asterisk_stack.is_empty() {
                        return false;
                    }
                    if !left_bracket_stack.is_empty() {
                        left_bracket_stack.pop();
                    } else {
                        asterisk_stack.pop();
                    }
                }
            }
        }
        while !left_bracket_stack.is_empty() && !asterisk_stack.is_empty() {
            let i = left_bracket_stack.pop().unwrap();
            let j = asterisk_stack.pop().unwrap();
            if i.0 > j.0 {
                return false;
            }
        }

        if left_bracket_stack.is_empty() {
            return true;
        } else {
            false
        }
    }
}






// 贪心
// 可以在遍历过程中维护未匹配的左括号数量可能的最小值和最大值，根据遍历到的字符更新最小值和最大值：
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min_cnt = 0;
        let mut max_cnt = 0;
        let len = s.len();
        let s_chars: Vec<char> = s.chars().collect();
        for b in s.bytes() {
            match b {
                b'(' => {
                    min_cnt += 1;
                    max_cnt += 1;
                },
                b')' => {
                    min_cnt = (min_cnt-1).max(0);
                    max_cnt -= 1;
                    if max_cnt < 0 {
                        return false;
                    }
                },
                _ => {
                    min_cnt = (min_cnt-1).max(0);
                    max_cnt += 1;
                }
            }
        }
        min_cnt == 0
    }
}




/// 看到题目容易想到位运算吧
/// 128位在其它语言里可能需要特殊处理，但 rust 里直接用就可以了
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut tmp = 1u128;
        for b in s.bytes() {
            match b {
                b'(' => tmp <<= 1,
                b')' => tmp >>= 1,
                _ => tmp |= (tmp << 1) | (tmp >> 1)
            }
        }
        (tmp & 1) != 0
    }
}
  



impl Solution {
    /// ## 解题思路
    /// - 贪心法
    ///
    pub fn check_valid_string(s: String) -> bool {
        let mut unpaired_lefts = 0; //未匹配的'('数
        let mut can_pair_rights = 0; //可匹配')'的容量
        for c in s.chars() {
            match c {
                '(' => {
                    // 可匹配')'的数+1
                    can_pair_rights += 1;
                    // 未匹配的'('总数+1
                    unpaired_lefts += 1;
                }
                ')' => {
                    // 如果没有匹配')'的容量
                    if can_pair_rights <= 0 {
                        return false;
                    } else {
                        can_pair_rights -= 1;
                    }
                    // 如果存在未匹配的'('
                    if unpaired_lefts > 0 {
                        //则将未匹配的'('-1
                        unpaired_lefts -= 1;
                    }
                }
                '*' => {
                    // 可匹配')'的容量+1
                    can_pair_rights += 1;
                    // 如果有未匹配的'('
                    if unpaired_lefts > 0 {
                        // 则未匹配的'('计数-1
                        unpaired_lefts -= 1;
                    };
                }
                _ => {}
            }
        }
        unpaired_lefts == 0
    }
}