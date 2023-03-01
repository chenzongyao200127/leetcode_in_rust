// 1792. Maximum Average Pass Ratio
// https://leetcode.cn/problems/maximum-average-pass-ratio/

impl Solution {
    fn less(lhs: [i32; 2], rhs: [i32; 2]) -> bool {
        // (lhs[0]+1)/(lhs[1]+1) -lhs[0]/lhs[1]
        let first = (lhs[0] + 1) as f64 / (lhs[1] + 1) as f64 - lhs[0] as f64 / lhs[1] as f64;
        let second = (rhs[0] + 1) as f64 / (rhs[1] + 1) as f64 - rhs[0] as f64 / rhs[1] as f64;
        first > second
    }

    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut h = [[0; 2]; 100005];
        for (i, class) in classes.iter().enumerate() {
            h[i] = [class[0], class[1]];
            let mut k = i;
            while k > 0 && Self::less(h[k], h[(k - 1) / 2]) {
                let temp = h[(k - 1) / 2];
                h[(k - 1) / 2] = h[k];
                h[k] = temp;
                k = (k - 1) / 2;
            }
        }

        let n = classes.len();
        for _ in 0..extra_students {
            h[0][0] += 1;
            h[0][1] += 1;
            let mut k = 0;
            while k < n {
                let mut opt = k;
                for next in 2 * k + 1..=2 * k + 2 {
                    if next < n && Self::less(h[next], h[opt]) {
                        opt = next;
                    }
                }

                if opt == k {
                    break;
                }
                let temp = h[opt];
                h[opt] = h[k];
                h[k] = temp;
                k = opt;
            }
            // sink.
        }

        let mut res = 0.0;
        for v in h.iter().take(classes.len()) {
            res += v[0] as f64 / v[1] as f64;
        }
        res / n as f64
    }
}


#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;
        #[inline]
        fn score(pass: i32, total: i32) -> F64 {
            let (p, t) = (pass as f64, total as f64);
            let a = (p + 1.0) / (t + 1.0);
            F64(a - p / t)
        }
        let len = classes.len();
        let mut classes: BinaryHeap<_> = classes
            .into_iter()
            .map(|c| (score(c[0], c[1]), c[0], c[1]))
            .collect();
        for _ in 0..extra_students {
            if let Some((_, p, t)) = classes.pop() {
                let (p, t) = (p + 1, t + 1);
                classes.push((score(p, t), p, t));
            }
        }
        let mut ans = 0.0;
        while let Some((_, p, t)) = classes.pop() {
            ans += p as f64 / t as f64;
        }
        ans / len as f64
    }
}


impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;
    
        fn ratio_increment(pass: i32, total: i32) -> i32 {
            const FACTOR: f64 = 10000000.0;
    
            let pass = pass as f64;
            let total = total as f64;
            (FACTOR * ((pass + 1.0) / (total + 1.0) - pass / total)) as i32
        }
    
        let mut pq = BinaryHeap::new();
        for class in classes.iter() {
            let (pass, total) = (class[0], class[1]);
            let ratio_diff = ratio_increment(pass, total);
            pq.push((ratio_diff, pass, total));
        }
    
        for _ in 0..extra_students {
            if let Some((_, mut pass, mut total)) = pq.pop() {
                pass += 1;
                total += 1;
                let ratio_diff = ratio_increment(pass, total);
                pq.push((ratio_diff, pass, total));
            }
        }
    
        let mut sum = 0.0;
        for &(_, pass, total) in pq.iter() {
            sum += pass as f64 / total as f64;
        }
    
        sum / classes.len() as f64
    }
}