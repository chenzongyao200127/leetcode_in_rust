// 319. Bulb Switcher
// https://leetcode.cn/problems/bulb-switcher/

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

// Time Out
pub fn bulb_switch(n: i32) -> i32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut bulds = vec![1; n as usize];
    for i in 1..n {
        for j in 0..bulds.len() {
            if (j+1) % (i+1) as usize == 0 {
                bulds[j] = 1-bulds[j];
            }
        }
        // println!("{:?}", bulds);
    }

    bulds.into_iter().sum::<i32>()
}