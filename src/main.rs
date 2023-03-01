use std::collections::{HashSet, HashMap};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = largest_local(vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]]);
    assert_eq!(ans, vec![vec![2,2,2],vec![2,2,2],vec![2,2,2]]);

}

pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = grid.len();
    let mut ans = vec![vec![0; len-2]; len-2];
    for i in 2..len {
        for j in 2..len {
            ans[i-2][j-2] = largest_nine(&grid, i, j);
        }
    }
    ans
}


pub fn largest_nine(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut max = 0;
    for i in x-2..=x {
        for j in y-2..=y {
            max = max.max(grid[i][j]);
        }
    }

    max
}