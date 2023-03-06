// 204. Count Primes
// https://leetcode.cn/problems/count-primes/


impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }

        let mut primes = vec![true; n as usize];
        primes[0] = false;
        primes[1] = false;
        for i in 2..=(n as f64).sqrt().floor() as usize {
            if primes[i] {
                let b = if i == 2 { 0 } else { 1 };
                for j in (i * i..n as usize).step_by(i << b) {
                    primes[j] = false;
                }
            }
        }
        primes.iter().map(|b| *b as i32).sum()
    }
}


impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 0;
        }
        
        let mut cnt_vec = vec![0; n as usize];
        for i in 2..(n as f32).sqrt().ceil() as usize {
            let mut multi = 2;
            while (multi * i as i32) <  n {
                cnt_vec[multi as usize * i] = 1;
                multi += 1;
            }
        }

        cnt_vec.iter().filter(|x| **x==0 ).count() as i32 - 2
    }
}