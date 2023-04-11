fn main() {
    let ans = is_robot_bounded("GL".to_string());
    assert_eq!(ans, true);
}

pub fn is_robot_bounded(instructions: String) -> bool {
    // direction: '北': 0 '西': 1 '南': 2, '东': 3
    let mut location = ((0, 0), 0);
    let instructions: Vec<char> = instructions.chars().collect();

    for &ins in instructions.iter() {
        match ins {
            'G' => {
                match location.1 {
                    0 => {
                        location.0.1 += 1;
                    },
                    1 => {
                        location.0.0 -= 1;
                    }
                    2 => {
                        location.0.1 -= 1;
                    }
                    _ => {
                        location.0.0 += 1;
                    }
                }
            }
            'L' => {
                location.1 = (location.1 + 5) % 4;
            },
             _  => {
                location.1 = (location.1 + 3) % 4;
            }
        }
    }

    location.0 == (0, 0) || location.1 != 0
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