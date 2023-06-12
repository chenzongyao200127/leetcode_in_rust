// 401. 二进制手表
// https://leetcode.cn/problems/binary-watch/

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            nums: vec![1, 2, 4, 8, 1, 2, 4, 8, 16, 32],
        }
    }

    pub fn read_binary_watch(&self, num: i32) -> Vec<String> {
        fn dfs(
            solution: &Solution,
            num: i32,
            step: i32,
            start: usize,
            visited: &mut Vec<bool>,
            result_all: &mut Vec<String>,
        ) {
            if step == num {
                let (hour, minute) = calc_time(solution, visited);
                if 0 <= hour && hour <= 11 && 0 <= minute && minute <= 59 {
                    result_all.push(format!("{}:{:02}", hour, minute));
                }
                return;
            }
            for i in start..solution.nums.len() {
                visited[i] = true;
                dfs(solution, num, step + 1, i + 1, visited, result_all);
                visited[i] = false;
            }
        }

        fn calc_time(solution: &Solution, visited: &Vec<bool>) -> (i32, i32) {
            let hour = solution
                .nums
                .iter()
                .enumerate()
                .filter(|&(i, _)| i < 4 && visited[i])
                .map(|(_, &num)| num)
                .sum();
            let minute = solution
                .nums
                .iter()
                .enumerate()
                .filter(|&(i, _)| i >= 4 && visited[i])
                .map(|(_, &num)| num)
                .sum();
            (hour, minute)
        }

        let mut result_all = Vec::new();
        dfs(
            &self,
            num,
            0,
            0,
            &mut vec![false; self.nums.len()],
            &mut result_all,
        );
        result_all
    }
}


impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nums = vec![1, 2, 4, 8, 1, 2, 4, 8, 16, 32];
        let mut result_all = Vec::new();
        dfs(
            turned_on,
            0,
            0,
            &mut vec![false; nums.len()],
            &mut result_all,
        );

        result_all
    }
}

pub fn dfs(
    turned_on: i32,
    step: i32,
    start: usize,
    visited: &mut Vec<bool>,
    result_all: &mut Vec<String>,
) {
    if step == turned_on {
        let (hour, minute) = calc_time(visited);
        if 0 <= hour && hour <= 11 && 0 <= minute && minute <= 59 {
            result_all.push(format!("{}:{:02}", hour, minute));
        }
        return;
    }
    for i in start..solution.nums.len() {
        visited[i] = true;
        dfs(turned_on, step + 1, i + 1, visited, result_all);
        visited[i] = false;
    }
}

pub fn calc_time(visited: &Vec<bool>) -> (i32, i32) {
    let hour = nums
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < 4 && visited[i])
        .map(|(_, &num)| num)
        .sum();
    let minute = solution
        .nums
        .iter()
        .enumerate()
        .filter(|&(i, _)| i >= 4 && visited[i])
        .map(|(_, &num)| num)
        .sum();
    (hour, minute)
}