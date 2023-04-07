// 1040_Moving_Stones_Until_Consecutive_II
// https://leetcode.cn/problems/moving-stones-until-consecutive-ii/

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let len = stones.len();
        let mut stones = stones;
        stones.sort_unstable();

        let mut min_moves = i32::MAX;
        let max_moves = (stones[len-1] - stone[1] - n + 2).max(stones[len-2] - stone[0] - n + 2);

        let mut i = 0;
        for j in 0..len {
            while (stones[j] - stones[i]) >= n {
                i += 1;
            }
            if j - i + 1 == len - 1 && stones[j] - stones[i] == len - 2 {
                min_moves = min_moves.min(2);
            } else {
                min_moves = min_moves.min(len - (j - i + 1))
            }
        }
        
        vec![min_moves, max_moves]
    }
}

// class Solution:
//     def numMovesStonesII(self, stones: List[int]) -> List[int]:
//         stones.sort()
//         n = len(stones)
//         min_moves = float("inf")
//         max_moves = max(stones[-1] - stones[1] - n + 2, stones[-2] - stones[0] - n + 2)

//         i = 0
//         for j in range(n):
//             while stones[j] - stones[i] >= n:
//                 i += 1
//             if j - i + 1 == n - 1 and stones[j] - stones[i] == n - 2:
//                 min_moves = min(min_moves, 2)
//             else:
//                 min_moves = min(min_moves, n - (j - i + 1))

//         return [min_moves, max_moves]


pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    let mut res = vec![i32::MAX, i32::MIN];
    let mut stones = stones;

    stones.sort_unstable();

    let mut stones_vec = vec![0; (stones[stones.len()-1] - stones[0] + 1) as usize];

    for stone in stones.iter() {
        stones_vec[(stone - stones[0]) as usize] = 1;
    }
    println!("ori:{:?}", stones_vec);

    let times = 0;
    dfs_moving_stones(stones_vec, &mut res, times);

    res
}

pub fn dfs_moving_stones(stones_vec: Vec<i32>, res: &mut Vec<i32>, times: i32) {
    let mut flag = true;
    for stone in stones_vec.iter() {
        if *stone == 0 {
            flag = false;
        }
    }
    if flag {
        res[0] = res[0].min(times);
        res[1] = res[1].max(times);
        return;
    }

    let mut start_idx = 0;
    while stones_vec[start_idx] == 1 {
        start_idx += 1;
    }
    let mut optional_spaces_for_start = vec![];
    for i in start_idx..stones_vec.len() {
        if stones_vec[i] == 0 {
            optional_spaces_for_start.push(i);
        }
    }
    println!("1.spaces:{:?}", optional_spaces_for_start);

    let mut end_idx = stones_vec.len() - 1;
    while stones_vec[end_idx] == 1 {
        end_idx -= 1;
    }
    let mut optional_spaces_for_end = vec![];
    for i in (0..=end_idx).rev() {
        if stones_vec[i] == 0 {
            optional_spaces_for_end.push(i);
        }
    }
    println!("2.spaces{:?}", optional_spaces_for_end);

    for space_idx in optional_spaces_for_start {
        let mut tmp_stones = stones_vec.clone();
        tmp_stones[space_idx] = 1;
        tmp_stones[0] = 0;
        while tmp_stones[0] == 0 {
            tmp_stones.remove(0);
        }
        println!("1:{:?}", tmp_stones);
        dfs_moving_stones(tmp_stones, res, times+1)
    }


    for space_idx in optional_spaces_for_end {
        let mut tmp_stones = stones_vec.clone();
        tmp_stones[space_idx] = 1;
        tmp_stones.pop();
        while tmp_stones[tmp_stones.len()-1] == 0 {
            tmp_stones.pop();
        }
        println!("2:{:?}", tmp_stones);
        dfs_moving_stones(tmp_stones, res, times+1)
    }

}