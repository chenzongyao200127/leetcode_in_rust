// 202. Happy Number
// https://leetcode.cn/problems/happy-number/

// 最终会得到 1
// 最终会进入循环。
// 值会越来越大，最后接近无穷大。(即使在代码中你不需要处理第三种情况，你仍然需要理解为什么它永远不会发生，这样你就可以证明为什么你不处理它。)
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut map = vec![0;9];
        for i in 1..map.len() {
            map[i] = i * i;
        }
        let mut sum = 0;
        let mut num = n;
        let mut set: HashSet<i32> = HashSet::new();
        while sum != 1 {
            sum = 0;
            while num != 0 {
                let unit = num % 10;
                num = num / 10;
                sum += unit * unit;
            }
            if let Some(_) = set.get(&sum) {
                return false
            } else {
                set.insert(sum);
            }
            num = sum;
        }

        true
    }
}

//  参考英文网站热评第一。这题可以用快慢指针的思想去做，有点类似于检测是否为环形链表那道题
//  如果给定的数字最后会一直循环重复，那么快的指针（值）一定会追上慢的指针（值），也就是
//  两者一定会相等。如果没有循环重复，那么最后快慢指针也会相等，且都等于1。
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        #[inline]
        fn next(v: &mut i32) {
            let (mut m, mut n) = (*v, 0_i32);
            while m > 0 {
                let t = m % 10;
                n += t * t;
                m /= 10;
            }
            *v = n;
        }
        let (mut q, mut s) = (n, n);
        loop {
            next(&mut s);
            next(&mut q);
            next(&mut q);
            if s == q {
                return q==1;
            }
        }
    }
}