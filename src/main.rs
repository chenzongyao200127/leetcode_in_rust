use std::collections::{HashSet, VecDeque};
use itertools::iproduct;

// 这是一个类似于深度优先搜索（DFS）的问题，
// 因为我们需要尝试所有可能的球的插入方式，直到找到一个成功的方案。在这个问题中，我们可以使用递归和回溯的方法来实现。
// 首先，我们需要实现一个方法来模拟球的插入和移除过程。
// 然后，我们在这个方法中使用 DFS 来尝试所有可能的插入方式。
// 在每次尝试插入一个球后，我们需要更新 board 和 hand 的状态，并进行回溯。
// 如果在某次尝试后，board 变为空，我们就找到了一个成功的方案，此时我们更新我们的答案。
// 这个问题比较复杂，涉及到很多细节，需要特别小心。下面是一个可能的解决方案：

fn clean(board: String) -> String {
    let mut stack: Vec<(char, usize, i32)> = Vec::new();
    let mut i = 0;
    while i < board.len() {
        if stack.is_empty() || stack.last().unwrap().0 != board.chars().nth(i).unwrap() {
            stack.push((board.chars().nth(i).unwrap(), i, 1));
        } else {
            let last = stack.last_mut().unwrap();
            last.2 += 1;
            if last.2 == 3 {
                stack.pop();
            }
        }
        i += 1;
    }
    stack.iter().map(|(ch, _, cnt)| ch.to_string().repeat((*cnt) as usize)).collect::<Vec<_>>().join("")
}

pub fn find_min_step(board: String, hand: String) -> i32 {
    let mut hand: Vec<char> = hand.chars().collect();
    hand.sort_unstable();
    let hand = hand.into_iter().collect::<String>();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // 初始化用队列维护的状态队列：其中的三个元素分别为桌面球状态、手中球状态和回合数
    queue.push_back((board.clone(), hand.clone(), 0));

    // 初始化用哈希集合维护的已访问过的状态
    visited.insert((board.clone(), hand.clone()));

    while let Some((cur_board, cur_hand, step)) = queue.pop_front() {
        for (i, j) in iproduct!(0..=cur_board.len(), 0..cur_hand.len()) {
            // println!("{:?}", (i,j));
            // 第 1 个剪枝条件: 当前球的颜色和上一个球的颜色相同
            if j > 0 && cur_hand.chars().nth(j).unwrap() == cur_hand.chars().nth(j - 1).unwrap() {
                continue;
            }

            // 第 2 个剪枝条件: 只在连续相同颜色的球的开头位置插入新球
            if i > 0 && cur_board.chars().nth(i - 1).unwrap() == cur_hand.chars().nth(j).unwrap() {
                continue;
            }

            // 第 3 个剪枝条件: 只在以下两种情况放置新球
            let mut choose = false;
            
            // 第 1 种情况 : 当前球颜色与后面的球的颜色相同
            if i < cur_board.len() 
                && cur_board.chars().nth(i).unwrap() == cur_hand.chars().nth(j).unwrap() {
                choose = true;
            }
            
            // 第 2 种情况 : 当前后颜色相同且与当前颜色不同时候放置球
            if 0 < i && i < cur_board.len() 
                && cur_board.chars().nth(i - 1).unwrap() == cur_board.chars().nth(i).unwrap() 
                && cur_board.chars().nth(i - 1).unwrap() != cur_hand.chars().nth(j).unwrap() {
                choose = true;
            }

            if choose {
                let new_board = clean(format!("{}{}{}", &cur_board[..i], 
                    cur_hand.chars().nth(j).unwrap(), 
                    &cur_board[i..]));
                let new_hand = format!("{}{}", &cur_hand[..j], &cur_hand[j + 1..]);
                if new_board.is_empty() {
                    return step + 1;
                }
                if !visited.contains(&(new_board.clone(), new_hand.clone())) {
                    queue.push_back((new_board.clone(), new_hand.clone(), step + 1));
                    visited.insert((new_board.clone(), new_hand.clone()));
                }
            }
        }
    }
    -1
}

pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
    if final_sum % 2 != 0 {
        return vec![];
    }

    let mut tmp = 0;
    let mut res: Vec<i64> = vec![];
    while tmp < final_sum {
        tmp += 2;
        final_sum -= tmp;
        res.push(tmp);
    }
    *res.last_mut().unwrap() += final_sum;
    return res
}

fn main() {
    let ans = find_min_step("WWRRBBWW".to_string(), "WWRB".to_string());
    assert_eq!(ans, 2);
}