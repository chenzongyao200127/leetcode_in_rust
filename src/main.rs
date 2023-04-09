fn main() {
    // let ans = num_moves_stones_ii(vec![7,4,9]);
    // assert_eq!(ans, vec![1,2]);

    // let ans = num_moves_stones_ii(vec![6,5,4,3,10]);
    // assert_eq!(ans, vec![2,3]);

    // let ans = num_moves_stones_ii(vec![100,101,104,102,103]);
    // assert_eq!(ans, vec![0,0]);

    let ans = check_distances("abaccb".to_string(), vec![1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert_eq!(ans, true);
}

pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut idx_vec: Vec<i32> = vec![-1; 26];
    let s_chars: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        if idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] == -1 {
            idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] = i as i32;
        } else {
            idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] = i as i32 - idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] - 1;
        }
    }
    // println!("{:?}", idx_vec);
    for i in 0..distance.len() {
        if idx_vec[i] == -1 {
            continue;
        } else {
            if idx_vec[i] != distance[i] {
                return false;
            }
        }
    }

    true
}