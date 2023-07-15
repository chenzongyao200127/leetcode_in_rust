// 65_Valid_Number
// https://leetcode.cn/problems/valid-number/solutions/8041/biao-qu-dong-fa-by-user8973/


// / 首先定义了一个枚举 State 来表示所有的状态。
// / 然后遍历输入的字符串，根据当前字符和当前状态来决定下一个状态。
// / 如果不能根据当前字符和当前状态找到下一个状态，那么就返回 false。
// / 最后，我们的最后一个状态是 Integer（整数部分）、Fraction（小数部分）、
// / ExpNum（指数部分）或 End（结束状态），那么我们就返回 true，否则返回 false。

impl Solution {
    pub fn is_number(s: String) -> bool {
        enum State {
            Start,
            Sign,
            Integer,
            Point,
            Fraction,
            Exp,
            ExpSign,
            ExpNum,
            End,
        }
    
        let mut state = State::Start;
        for c in s.chars() {
            state = match state {
                State::Start => match c {
                    ' ' => State::Start,
                    '+' | '-' => State::Sign,
                    '0'..='9' => State::Integer,
                    '.' => State::Point,
                    _ => return false,
                },
                State::Sign => match c {
                    '0'..='9' => State::Integer,
                    '.' => State::Point,
                    _ => return false,
                },
                State::Integer => match c {
                    '0'..='9' => State::Integer,
                    '.' => State::Fraction,
                    'e' | 'E' => State::Exp,
                    ' ' => State::End,
                    _ => return false,
                },
                State::Point => match c {
                    '0'..='9' => State::Fraction,
                    _ => return false,
                },
                State::Fraction => match c {
                    '0'..='9' => State::Fraction,
                    'e' | 'E' => State::Exp,
                    ' ' => State::End,
                    _ => return false,
                },
                State::Exp => match c {
                    '+' | '-' => State::ExpSign,
                    '0'..='9' => State::ExpNum,
                    _ => return false,
                },
                State::ExpSign => match c {
                    '0'..='9' => State::ExpNum,
                    _ => return false,
                },
                State::ExpNum => match c {
                    '0'..='9' => State::ExpNum,
                    ' ' => State::End,
                    _ => return false,
                },
                State::End => match c {
                    ' ' => State::End,
                    _ => return false,
                },
            };
        }
    
        match state {
            State::Integer | State::Fraction | State::ExpNum | State::End => true,
            _ => false,
        }
    }    
}
