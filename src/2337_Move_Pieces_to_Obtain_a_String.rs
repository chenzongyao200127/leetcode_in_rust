// 2337_Move_Pieces_to_Obtain_a_String
// https://leetcode.cn/problems/move-pieces-to-obtain-a-string/

// You are given two strings start and target, both of length n. Each string consists only of the characters 'L', 'R', and '_' where:

// The characters 'L' and 'R' represent pieces, where a piece 'L' can move to the left only if there is a blank space directly to its left, and a piece 'R' can move to the right only if there is a blank space directly to its right.
// The character '_' represents a blank space that can be occupied by any of the 'L' or 'R' pieces.
// Return true if it is possible to obtain the string target by moving the pieces of the string start any number of times. Otherwise, return false.

// 超时

/// The target.chars().nth(j) call is the cause of the timeout. 
/// In Rust, strings are UTF-8 encoded, and accessing a character by its index using nth() is an O(n) operation 
/// since it has to iterate over the bytes to determine the nth character due to variable-length encoding. 
/// Therefore, within the loop, repeatedly calling nth() can cause the algorithm to degrade into O(n^2) complexity.
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        if start.replace("_", "") != target.replace("_", "") {
            return false;
        }

        let mut j = 0;
        for (i, c) in start.chars().enumerate() {
            if c == '_' {
                continue;
            }
            while target.chars().nth(j).unwrap_or(' ') == '_' {
                j += 1;
            }
            if i != j && (c == 'L') == (i < j) {
                return false;
            }
            j += 1;
        }
        true
    }
}


impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        if start.replace("_", "") != target.replace("_", "") {
            return false;
        }

        let mut j = 0;
        let mut target_chars = target.chars().peekable();
        for (i, c) in start.chars().enumerate() {
            if c == '_' {
                continue;
            }
            
            while target_chars.peek() == Some(&'_') {
                target_chars.next();
                j += 1;
            }
            
            if i != j && (c == 'L') == (i < j) {
                return false;
            }
            target_chars.next();
            j += 1;
        }
        
        true
    }
}