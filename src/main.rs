use std::collections::{HashMap, HashSet};
pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {

    let mut prev: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
    for rela in relations {
        prev[rela[1] as usize].push(rela[0].clone());
    }
    
    let mut memo: HashMap<i32, i32> = HashMap::new();
    
    #[inline]
    fn dfs(i: i32, memo: &mut HashMap<i32, i32>, prev: &Vec<Vec<i32>>, time: &Vec<i32>) -> i32 {
        if let Some(v) = memo.get(&i) {
            return *v
        }

        let mut cur = 0;
        for pre in prev[i as usize].iter() {
            cur = cur.max(dfs(*pre, memo, prev, time));
        }
        cur += time[i as usize - 1];
        memo.insert(i, cur);
        return cur
    }

    let mut ans = 0;
    (1..=n).into_iter().for_each(|x| ans = ans.max(dfs(x, &mut memo, &prev, &time)));
    ans
}

pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut dp = vec![vec![vec![0; n as usize]; n as usize]; k as usize + 1];
    dp[0][row as usize][column as usize] = 1;
    let mut out_counts = 0;
    let mut stay_counts = 0;

    for i in 0..k {
        for j in 0..n {
            for k in 0..n {
                if dp[i as usize][j as usize][k as usize] > 0 {
                    for &(dx, dy) in [(-2, -1), (-1, -2), (1, 2), (2, 1), 
                                      (-2, 1), (-1, 2), (1, -2), (2, -1)].iter() {
                        let new_x = j + dx;
                        let new_y = k + dy;
                        if new_x >= 0 && new_x < n && new_y >= 0 && new_y < n {
                            dp[i as usize + 1][new_x as usize][new_y as usize] = 
                                dp[i as usize + 1][new_x as usize][new_y as usize] 
                                    + dp[i as usize][j as usize][k as usize];
                        } else {
                            out_counts = out_counts + dp[i as usize][j as usize][k as usize];
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            stay_counts += dp[k as usize][i as usize][j as usize];
        }
    }

    return (out_counts) as f64 / (out_counts + stay_counts) as f64
}

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mut dp = vec![vec![vec![0; n as usize]; m as usize]; max_move as usize + 1];
    dp[0][start_row as usize][start_column as usize] = 1;
    let mut out_counts = 0;
    let mod_num = 1_000_000_007;

    for i in 0..max_move {
        for j in 0..m {
            for k in 0..n {
                if dp[i as usize][j as usize][k as usize] > 0 {
                    for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                        let new_x = j + dx;
                        let new_y = k + dy;
                        if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n {
                            dp[i as usize + 1][new_x as usize][new_y as usize] = 
                                (dp[i as usize + 1][new_x as usize][new_y as usize] 
                                    + dp[i as usize][j as usize][k as usize]) % mod_num;
                        } else {
                            out_counts = (out_counts + dp[i as usize][j as usize][k as usize]) % mod_num;
                        }
                    }
                }
            }
        }
    }
    out_counts
}

fn main() {
    let ans = minimum_time(3, vec![vec![1,3],vec![2,3]], vec![3,2,5]);
    assert_eq!(ans, 4);
}

pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        for j in 1..i {
            dp[i] += dp[j-1] * dp[i-j];
        }
    }

    dp[n]
}

pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let m = dungeon.len();
    let n = dungeon[0].len();

    let mut dp = vec![vec![1_000_000_007; n + 1]; m + 1];
    dp[m][n-1] = 1;
    dp[m-1][n] = 1;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let tmp = dp[i][j].min(dp[i+1][j].min(dp[i][j+1]));
            dp[i][j] = (1).max(tmp - dungeon[i][j]);
        }
    }

    dp[0][0]
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let len = get_list_len(head.as_ref());
    if len <= 2 { return; }
    
    let (prev, next) = split_list(head.take(), len);
    let reverse_next = reverse(next);
    *head = merge(prev, reverse_next);
}

fn get_list_len(mut head: Option<&Box<ListNode>>) -> usize {
    let mut len = 0;
    while let Some(node) = head.as_ref() {
        head = node.next.as_ref();
        len += 1;
    }
    len   
}

fn split_list(mut head: Option<Box<ListNode>>, len: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mid = (len + 1) / 2;
    let mut i = 1;
    let mut cur = head.as_mut().unwrap();

    while i < mid {
        cur = cur.next.as_mut().unwrap();
        i += 1;
    }

    let next = cur.next.take();
    (head, next)
}

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reverse_head = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = reverse_head.take();
        reverse_head = Some(node);
    }
    
    reverse_head
}

fn merge(mut head1: Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) ->Option<Box<ListNode>> {
    let mut cur = head1.as_mut().unwrap();
    while let Some(mut node) = head2 {
        head2 = node.next.take();
        node.next = cur.next.take();
        cur.next = Some(node);

        match cur.next.as_mut().unwrap().next.as_mut() {
            Some(next) => cur = next,
            None => break,
        }
    }

    head1
}

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    #[inline]
    fn dfs(matrix: &Vec<Vec<i32>>, cur_x: i32, cur_y: i32, length: i32, memo: &mut HashSet<(i32, i32)>) -> i32 {
        let dirs = vec![(1,0), (0,1), (-1,0), (0,-1)];
        let mut max_length = length;
        for (dx, dy) in dirs {
            let new_x = cur_x + dx;
            let new_y = cur_y + dy;

            if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32 
                    && !memo.contains(&(new_x, new_y)) 
                    && matrix[new_x as usize][new_y as usize] > matrix[cur_x as usize][cur_y as usize] {
                memo.insert((new_x, new_y));
                max_length = max_length.max(dfs(matrix, new_x, new_y, length + 1, memo));
                memo.remove(&(new_x, new_y));
            }
        }
        max_length
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let mut memo: HashSet<(i32, i32)> = HashSet::new();
            memo.insert((i as i32, j as i32));
            ans = ans.max(dfs(&matrix, i as i32, j as i32, 1, &mut memo));
        }
    }

    ans
}
