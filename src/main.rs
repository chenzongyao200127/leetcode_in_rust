fn main() {
    let ans = num_moves_stones_ii(vec![7,4,9]);
    assert_eq!(ans, vec![1,2]);

    let ans = num_moves_stones_ii(vec![6,5,4,3,10]);
    assert_eq!(ans, vec![2,3]);

    let ans = num_moves_stones_ii(vec![100,101,104,102,103]);
    assert_eq!(ans, vec![0,0]);
}

pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    let len = stones.len();
    let mut stones = stones;
    stones.sort_unstable();

    let mut min_moves = i32::MAX;
    let max_moves = (stones[len-1] - stones[1] - len as i32 + 2).max(stones[len-2] - stones[0] - len as i32 + 2);

    let mut i = 0;
    for j in 0..len {
        while (stones[j] - stones[i]) >= len as i32 {
            i += 1;
        }
        if j - i + 1 == len - 1 && stones[j] - stones[i] == len as i32 - 2 {
            min_moves = min_moves.min(2);
        } else {
            min_moves = min_moves.min(len as i32 - (j - i + 1) as i32)
        }
    }
    
    vec![min_moves, max_moves]
}