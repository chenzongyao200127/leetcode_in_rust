// 554. Brick Wall
// https://leetcode.cn/problems/brick-wall/

use std::collections::HashMap;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut other_wall = vec![];
        let len = wall.len();
        let mut ans = wall.len();
        let mut long = 0;
        for line in wall {
            let mut cur_xum = 0;
            let mut new_line = vec![];
            for brick in line {
                cur_xum += brick;
                new_line.push(cur_xum);
            }
            long = cur_xum;
            other_wall.push(new_line);
        }
        let mut map: HashMap<i32, usize> = HashMap::new();
        for line in other_wall {
            for brick in line {
                map.entry(brick).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
        }
        for (&k, &v) in map.iter() {
            if k != long as i32 {
                ans = ans.min((len as usize - v) as usize);
            }
        }

        ans as i32
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let mut cnt = 0;
        let mut total = HashMap::new();
        for i in 0..n {
            let mut sum = 0;
            let m = wall[i].len();
            for j in 0..m - 1 {
                sum += wall[i][j];
                total.entry(sum).and_modify(|c| *c += 1).or_insert(1);
                cnt = cnt.max(total[&sum]);
            }
        }
        n as i32 - cnt
    }
}
