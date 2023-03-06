// 406. Queue Reconstruction by Height
// https://leetcode.cn/problems/queue-reconstruction-by-height/


// 方法一：从低到高考虑
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let len = people.len();
        people.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        let mut queue: Vec<Vec<i32>> = vec![vec![];len];
        for one in people.iter() {
            let x = one[0];
            let y = one[1];
            let mut cnt = y+1;
            for i in 0..queue.len() {
                if queue[i].is_empty() {
                    if cnt > 0 {
                        cnt -= 1;
                    }
                }
                if !queue[i].is_empty() && queue[i][0] >= x {
                    if cnt > 0 {
                        cnt -= 1;
                    }
                }
                if cnt == 0 {
                    if queue[i].is_empty() {
                        queue[i] = vec![x,y];
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }

        queue
    }
}

// 方法二：从高到低考虑
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let mut ans = vec![];
        people.sort_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        for p in people.iter() {
            ans.insert(p[1] as usize, p.to_vec())
        }
        ans
    }
}
