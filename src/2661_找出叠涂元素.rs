// 2661_找出叠涂元素
// https://leetcode.cn/problems/first-completely-painted-row-or-column/description/

use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        #[inline]
        fn find_position(n: i32, position_map: &HashMap<i32, (usize, usize)>)-> (usize, usize) {
            *position_map.get(&n).unwrap()
        }

        let mut row = vec![HashSet::new(); mat.len()];
        let mut col = vec![HashSet::new(); mat[0].len()];
        let mut position_map: HashMap<i32, (usize, usize)> = HashMap::new();

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                position_map.insert(mat[i][j], (i, j));
            }
        }

        for i in 0..arr.len() {
            let n = arr[i];
            let (x, y) = find_position(n, &position_map);
            row[x].insert(n);
            col[y].insert(n);
            // println!("{:?}", row[x]);
            // println!("{:?}", low[y]);
            if row[x].len() == mat[0].len() || col[y].len() == mat.len() {
                return i as i32;
            }
        }

        unimplemented!()
    }
}



impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut r = arr.len();

        let mut hash: std::collections::HashMap<i32, (usize, usize)> = Default::default();
        // s1存放每行的个数
        // s2存放没列的个数
        let (mut s1, mut s2) = (vec![0usize; mat.len()], vec![0usize; mat[0].len()]);

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                hash.insert(mat[i][j], (i, j));
            }
        }

        for i in 0..arr.len() {
            let (x, y) = hash[&arr[i]];

            s1[x] += 1usize;
            s2[y] += 1usize;

            if s1[x] == mat[0].len() || s2[y] == mat.len() {
                return i as i32;
            }
        }

        unreachable!()
    }
}



impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut p = vec![(0, 0); m * n + 1];

        mat.iter()
            .enumerate()
            .for_each(|(i, r)| 
                r.iter()
                    .enumerate()
                    .for_each(|(j, &val)|  p[val as usize] = (i, j))
            );

        let (mut rc, mut cc) = (vec![0; m], vec![0; n]);
        arr
        .into_iter()
        .enumerate()
        .find_map(|(i, val)| {
            let (r, c) = (p[val as usize].0, p[val as usize].1); 
            rc[r] += 1;
            cc[c] += 1;
            match rc[r] == n || cc[c] == m {
                true => Some(i as i32),
                false => None
            }
        }).unwrap()
    }
}
