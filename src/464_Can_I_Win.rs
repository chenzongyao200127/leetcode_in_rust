// 464_Can_I_Win
// https://leetcode.cn/problems/can-i-win/description/

// Example 1:

// Input: maxChoosableInteger = 10, desiredTotal = 11
// Output: false
// Explanation:
// No matter which integer the first player choose, the first player will lose.
// The first player can choose an integer from 1 up to 10.
// If the first player choose 1, the second player can only choose integers from 2 up to 10.
// The second player will win by choosing 10 and get a total = 11, which is >= desiredTotal.
// Same with other integers chosen by the first player, the second player will always win.

// Example 2:
// Input: maxChoosableInteger = 10, desiredTotal = 0
// Output: true

// Example 3:
// Input: maxChoosableInteger = 10, desiredTotal = 1
// Output: true

use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false;
        }
    
        if desired_total <= max_choosable_integer {
            return true;
        }
    
        fn dfs(
            state: i32, 
            cur_total: i32, 
            max_choosable_integer: i32, 
            desired_total: i32, 
            memo: &mut HashMap<i32, bool>
        ) -> bool {
            if let Some(&v) = memo.get(&state) {
                return v;
            }
    
            for i in (1..=max_choosable_integer).rev() {
                if (state & (1 << (i - 1))) == 0 {
                    if cur_total + i >= desired_total 
                        || !dfs(
                            state | (1 << (i - 1)), 
                            cur_total + i, 
                            max_choosable_integer, 
                            desired_total, 
                            memo
                        ) {
                        memo.insert(state, true);
                        return true;
                    }
                }
            }
    
            memo.insert(state, false);
            return false;
        }
    
        let mut memo = HashMap::new();
        dfs(0, 0, max_choosable_integer, desired_total, &mut memo)
    }    
}


// 继续优化
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false;
        }

        if desired_total <= max_choosable_integer {
            return true;
        }

        fn dfs(
            state: i32,
            cur_total: i32,
            max_choosable_integer: i32,
            desired_total: i32,
            memo: &mut Vec<Option<bool>>,
        ) -> bool {
            if let Some(v) = memo[state as usize] {
                return v;
            }

            for i in (1..=max_choosable_integer).rev() {
                if (state & (1 << (i - 1))) == 0 {
                    if cur_total + i >= desired_total
                        || !dfs(
                            state | (1 << (i - 1)),
                            cur_total + i,
                            max_choosable_integer,
                            desired_total,
                            memo,
                        ) {
                        memo[state as usize] = Some(true);
                        return true;
                    }
                }
            }

            memo[state as usize] = Some(false);
            return false;
        }

        let mut memo = vec![None; 1 << max_choosable_integer as usize];
        dfs(0, 0, max_choosable_integer, desired_total, &mut memo)
    }
}