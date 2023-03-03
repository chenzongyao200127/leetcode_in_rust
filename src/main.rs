use std::collections::HashMap;

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    // let ans = candy(vec![1,2,87,87,87,2,1]);
    // assert_eq!(ans, 13);
    let ans = merge(vec![vec![1,4],vec![5,6]]);
    assert_eq!(ans, vec![vec![1,6],vec![8,10],vec![15,18]]);
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut bitmap = vec![0;10001];
    let mut ans = vec![];
    for interval in intervals {
        let start = interval[0] as usize;
        let end = interval[1] as usize;
        for i in start..=end {
            bitmap[i] = 1;
        }    
    }
    let mut tmp = vec![];
    for i in 0..bitmap.len()-1 {
        if bitmap[i] != bitmap[i+1] {
            if bitmap[i] == 1 {
                tmp.push(i as i32);
            } else {
                tmp.push(i as i32 + 1);
            }
        }
    }
    for i in (0..tmp.len()-1).filter(|x| (x % 2 == 0)) {
        ans.push(vec![tmp[i], tmp[i+1]]);
    }

    ans
}

