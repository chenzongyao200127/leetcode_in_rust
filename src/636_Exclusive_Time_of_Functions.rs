// 636. Exclusive Time of Functions
// https://leetcode.cn/problems/exclusive-time-of-functions/
// 4 ms 100%
// 2.2 MB 100%
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut stk: Vec<i32> = vec![0];
        let mut cur_time = -1; 
        for log in logs.iter() {
            let log: Vec<&str> = log.split(':').collect();
            let func_id = log[0].parse::<i32>().unwrap();
            let op = log[1];
            let time = log[2].parse::<i32>().unwrap();
            
            if op == "start" {
                stk.push(func_id);
                let time = time - 1;
                ans[stk[stk.len()-2] as usize] = ans[stk[stk.len()-2] as usize] + (time-cur_time) as i32;
                cur_time = time;
            } else {
                ans[stk[stk.len()-1] as usize] = ans[stk[stk.len()-1] as usize] + (time-cur_time) as i32;
                cur_time = time;
                stk.pop();
            }
        }
    
        ans
    }
}