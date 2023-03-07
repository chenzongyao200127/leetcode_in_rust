pub fn main() {
    let ans = check_valid_string("(*)".to_string());
    assert_eq!(ans, true);
}

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