// 1222_Queens_That_Can_Attack_the_King
// https://leetcode.cn/problems/queens-that-can-attack-the-king/description/

// Input: queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]], king = [0,0]
// Output: [[0,1],[1,0],[3,3]]

// Input: queens = [[0,0],[1,1],[2,2],[3,4],[3,5],[4,4],[4,5]], king = [3,3]
// Output: [[2,2],[3,4],[4,4]]

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        #[inline]
        fn is_valid(x: i32, y: i32) -> bool {
            x < 8 && y < 8 && x >= 0 && y >= 0
        }
    
        let mut ans = vec![];
    
        let x_king = king[0];
        let y_king = king[1];
         
        let dirs = vec![(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

        for (dx, dy) in dirs {
            let mut new_x = x_king + dx;
            let mut new_y = y_king + dy;
            while is_valid(new_x, new_y) {
                if queens.contains(&vec![new_x, new_y]) {
                    ans.push(vec![new_x, new_y]);
                    break;
                }
                new_x = new_x + dx;
                new_y = new_y + dy;
            }
        }
    
        ans    
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        #[inline]
        fn is_valid(x: i32, y: i32) -> bool {
            x < 8 && y < 8 && x >= 0 && y >= 0
        }

        let mut ans = vec![];

        let x_king = king[0];
        let y_king = king[1];

        let queen_set: HashSet<(i32, i32)> = queens.into_iter().map(|q| (q[0], q[1])).collect();
        
        let dirs: [(i32, i32); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
        
        for (dx, dy) in dirs.iter() {
            let mut new_x = x_king + dx;
            let mut new_y = y_king + dy;
            while is_valid(new_x, new_y) {
                if queen_set.contains(&(new_x, new_y)) {
                    ans.push(vec![new_x, new_y]);
                    break;
                }
                new_x += dx;
                new_y += dy;
            }
        }

        ans
    }
}

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let (x_king, y_king) = (king[0], king[1]);

        for (dx, dy) in &[(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)] {
            let mut new_x = x_king + dx;
            let mut new_y = y_king + dy;

            while new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                if queens.contains(&vec![new_x, new_y]) {
                    ans.push(vec![new_x, new_y]);
                    break;
                }
                new_x += dx;
                new_y += dy;
            }
        }

        ans
    }
}