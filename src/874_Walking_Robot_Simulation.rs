// 874_Walking_Robot_Simulation
// https://leetcode.cn/problems/walking-robot-simulation/submissions/

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles_set: HashSet<(i32, i32)> = obstacles.into_iter()
            .map(|v| (v[0], v[1]))
            .collect();
        
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut cur_loc: (i32, i32) = (0, 0);
        let mut cur_dir: usize = 0;
        let mut ans: i32 = 0;
        
        for com in commands {
            if com == -1 {
                cur_dir = (cur_dir + 1) % 4;
            } else if com == -2 {
                cur_dir = (cur_dir + 3) % 4; // equivalent to cur_dir = (cur_dir - 1) % 4;
            } else {
                let (dx, dy) = directions[cur_dir];
                for _ in 0..com {
                    let new_loc = (cur_loc.0 + dx, cur_loc.1 + dy);
                    if obstacles_set.contains(&new_loc) {
                        break;
                    }
                    cur_loc = new_loc;
                }
                ans = ans.max(cur_loc.0 * cur_loc.0 + cur_loc.1 * cur_loc.1);
            }
        }
        
        ans
    }
}