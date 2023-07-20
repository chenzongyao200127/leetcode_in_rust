use std::collections::{HashSet, VecDeque};
use itertools::iproduct;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
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

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();

    fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<i32>>, directions: &[(i32, i32)]) -> i32 {
        let mut size = 1;
        grid[x][y] = 0;
        for &(dx, dy) in directions.iter() {
            let nx = dx + x as i32;
            let ny = dy + y as i32;
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 && grid[nx as usize][ny as usize] == 1 {
                size += dfs(nx as usize, ny as usize, grid, directions);
            }
        }
        size
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = ans.max(dfs(i, j, &mut grid.to_vec(), &directions));
            }
        }
    }

    ans
}

pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
    // 将每个工人的编号附加到 time 列表中，以便追踪
    let mut time: Vec<_> = time.into_iter().enumerate().map(|(i, mut t)| {
        t.push(i as i32);
        t
    }).collect();


    // 根据工人往返桥的时间对 time 进行排序
    time.sort_by_key(|x| (Reverse(x[0] + x[2]), Reverse(x[4])));

    let mut current_time = 0;
    let mut left_side_waiting = BinaryHeap::new();
    let mut right_side_waiting = BinaryHeap::new();
    let mut left_side_heap: BinaryHeap<_> = (0..k).map(|j| (Reverse(0), j)).collect();
    let mut right_side_heap = BinaryHeap::new();
    let mut boxes_left = n;
    let mut boxes_moved = 0;

    while boxes_left > 0 {
        if let Some((_, worker_idx)) = right_side_heap.pop() {
            left_side_waiting.push((Reverse(current_time + time[worker_idx as usize][2] + time[worker_idx as usize][3]), worker_idx));
            current_time += time[worker_idx as usize][2];
            boxes_left -= 1;
        } else if boxes_moved < n {
            let (_, worker_idx) = left_side_heap.pop().unwrap();
            right_side_waiting.push((Reverse(current_time + time[worker_idx as usize][0] + time[worker_idx as usize][1]), worker_idx));
            current_time += time[worker_idx as usize][0];
            boxes_moved += 1;
        } else {
            current_time = *[
                left_side_waiting.peek().map(|&(Reverse(t), _)| t).unwrap_or(10_i32.pow(9)),
                right_side_waiting.peek().map(|&(Reverse(t), _)| t).unwrap_or(10_i32.pow(9)),
            ].iter().min().unwrap();
        }

        while right_side_waiting.peek().map_or(false, |&(Reverse(t), _)| t <= current_time) {
            let (_, worker_idx) = right_side_waiting.pop().unwrap();
            right_side_heap.push((Reverse(worker_idx), current_time));
        }

        while left_side_waiting.peek().map_or(false, |&(Reverse(t), _)| t <= current_time) {
            let (_, worker_idx) = left_side_waiting.pop().unwrap();
            left_side_heap.push((Reverse(worker_idx), current_time));
        }
    }

    current_time
}


fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut max_sum = nums[0];
    let mut min_sum = nums[0];
    let mut curr_max = 0;
    let mut curr_min = 0;

    for num in nums {
        curr_max = (curr_max + num).max(num);
        max_sum = max_sum.max(curr_max);
        curr_min = (curr_min + num).min(num);
        min_sum = min_sum.min(curr_min);
        total += num;
    }
    if max_sum > 0 {
        [max_sum, total - min_sum].into_iter().max().unwrap()
    } else {
        max_sum
    }
}   

pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();

    let mut uf = UnionFind::new(n);

    let mut redundant = vec![];

    for edge in edges {
        let x = edge[0] as usize;
        let y = edge[1] as usize;

        let parent_x = uf.find(x);
        let parent_y = uf.find(y);

        if parent_x != parent_y {
            uf.union(parent_x, parent_y);
        } else {
            redundant = vec![x as i32, y as i32];
            break;
        }
    }

    redundant
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut end = 0;
    for i in 0..n {
        if i <= end {
            end = end.max(i + nums[i] as usize)
        }
        if end >= n - 1 {
            return true;
        }
    }
    false
}


// 这段代码的行为与原来的Python代码基本一致。不过由于Rust中元组的比较是从左到右进行的，
// 因此我们将区间长度和右边界的顺序颠倒，使得堆优先按照区间长度进行排序。
// 如果长度相同，再按照右边界排序。
// 此外，由于Rust中没有Python的列表解析，所以我们使用into_iter().enumerate().collect()来代替Python中的列表解析。


// 在Rust中，元组的比较是从左到右进行的，也就是说，
// 首先比较元组的第一个元素，如果它们相等，再比较第二个元素，依此类推。
// 因此，当我们把一个元组插入到堆中时，堆会先按照元组的第一个元素进行排序。
// 如果第一个元素相同，再按照第二个元素排序，以此类推。
// 所以，我们把区间长度放在元组的第一个位置，右边界放在第二个位置，这样就能保证堆优先按照区间长度进行排序，如果长度相同，再按照右边界排序。

// 至于Reverse，这是Rust中的一个包装器类型，用来改变某个值的排序顺序。
// 默认情况下，BinaryHeap是一个最大堆，也就是说，它总是把最大的元素放在堆顶。
// 但在这个问题中，我们需要一个最小堆，也就是总是把最小的元素放在堆顶。
// Rust的BinaryHeap并没有提供直接创建最小堆的方法，但我们可以通过Reverse包装器来实现这个功能。
// Reverse包装器会把它包装的值的排序顺序翻转，所以我们把区间长度和右边界都包装在Reverse中，再插入到堆中，就能得到一个最小堆。
pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut intervals = intervals;
    intervals.sort_by_key(|v| (v[0], -v[1]));

    let mut queries: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
    queries.sort_by_key(|&(_, x)| x);

    let mut heap = BinaryHeap::new();
    let mut res = vec![-1; queries.len()];
    let mut j = 0;
    for (i, qi) in queries {
        while j < intervals.len() && intervals[j][0] <= qi {
            let len = intervals[j][1] - intervals[j][0] + 1;
            heap.push((Reverse(len), Reverse(intervals[j][1])));
            j += 1;
        }
        while let Some((_, Reverse(right))) = heap.peek() {
            if *right < qi {
                heap.pop();
            } else {
                break;
            }
        }
        if let Some((Reverse(len), _)) = heap.peek() {
            res[i] = *len;
        }
    }
    res
}


pub fn alternate_digit_sum(n: i32) -> i32 {
    let n = n.to_string();
    let mut ans = 0;
    let mut flag = 1;
    n.chars().into_iter().for_each(|ch| {
        ans += (ch as u8 - '0' as u8) as i32 * flag;
        flag *= -1;
    });
    ans
}

fn main() {
    let ans = max_alternating_sum(vec![4,2,5,3,9,2,1,7,3,6]);
    assert_eq!(ans, 2);
}

