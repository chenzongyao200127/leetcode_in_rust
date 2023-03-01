use std::collections::{HashSet, HashMap};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = least_bricks(vec![vec![1,1],vec![2],vec![1,1]]);
    assert_eq!(ans, 1);

}

pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    let mut other_wall = vec![];
    let len = wall.len();
    let mut ans = wall.len();
    let mut long = 0;
    for line in wall {
        let mut cur_xum = 0;
        let mut new_line = vec![];
        for brick in line {
            cur_xum += brick;
            new_line.push(cur_xum);
        }
        long = cur_xum;
        other_wall.push(new_line);
    }
    let mut map: HashMap<i32, usize> = HashMap::new();
    for line in other_wall {
        for brick in line {
            map.entry(brick).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
    }
    for (&k, &v) in map.iter() {
        println!("{:?}",(k,v));
        if k != long as i32 {
            ans = ans.min((len as usize - v) as usize);
            println!("{:?}", ans);
        }
    }

    ans as i32
}