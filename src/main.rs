use std::collections::BinaryHeap;

pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let k = k as usize;
    let c = candidates as usize;
    if c * 2 + k > n {
        costs.sort_unstable();
        return costs[..k].iter().map(|&x| x as i64).sum();
    }

    let mut pre = BinaryHeap::new();
    let mut suf = BinaryHeap::new();
    for i in 0..c {
        pre.push(-costs[i]); // 加负号，变成最小堆
        suf.push(-costs[n - 1 - i]);
    }

    let mut ans = 0;
    let mut i = c;
    let mut j = n - 1 - c;
    for _ in 0..k {
        if pre.peek().unwrap() >= suf.peek().unwrap() {
            ans -= pre.pop().unwrap() as i64;
            pre.push(-costs[i]);
            i += 1;
        } else {
            ans -= suf.pop().unwrap() as i64;
            suf.push(-costs[j]);
            j -= 1;
        }
    }
    ans
}

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let n = quality.len();
    let k = k as usize;
    let mut id = (0..n).collect::<Vec<_>>();
    // 按照 r 值排序
    id.sort_unstable_by(|&i, &j| (wage[i] * quality[j]).cmp(&(wage[j] * quality[i])));

    let mut h = BinaryHeap::new();
    let mut sum_q = 0;
    for i in 0..k {
        h.push(quality[id[i]]);
        sum_q += quality[id[i]];
    }

    // 选 r 值最小的 k 名工人
    let mut ans = sum_q as f64 * wage[id[k - 1]] as f64 / quality[id[k - 1]] as f64;

    // 后面的工人 r 值更大
    // 但是 sum_q 可以变小，从而可能得到更优的答案
    for i in k..n {
        let q = quality[id[i]];
        if q < *h.peek().unwrap() {
            sum_q -= h.pop().unwrap() - q;
            h.push(q);
            ans = ans.min(sum_q as f64 * wage[id[i]] as f64 / q as f64);
        }
    }
    ans
}

// https://leetcode.cn/problems/maximum-profit-in-job-scheduling/description/?envType=daily-question&envId=2024-05-04
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = start_time
        .iter()
        .zip(end_time.iter())
        .zip(profit.iter())
        .map(|((&s, &e), &p)| (s, e, p))
        .collect::<Vec<_>>();
    jobs.sort_unstable_by(|a, b| a.1.cmp(&b.1)); // 按照结束时间排序

    let n = jobs.len();
    let mut f = vec![0; n + 1];
    for (i, &(st, _, p)) in jobs.iter().enumerate() {
        let j = jobs[..i].partition_point(|job| job.1 <= st);
        // 状态转移中，为什么是 j 不是 j+1：上面算的是 > st，-1 后得到 <= st，但由于还要 +1，抵消了
        f[i + 1] = f[i].max(f[j] + p);
    }
    f[n]
}

// 2391. 收集垃圾的最少总时间

use std::collections::HashSet;

pub fn calculate_garbage_collection_cost(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut total_cost = garbage[0].len() as i32;

    let mut unique_garbage_types = HashSet::new();

    for (garbage_at_house, &travel_distance) in garbage.iter().rev().zip(travel.iter().rev()) {
        unique_garbage_types.extend(garbage_at_house.chars());

        total_cost +=
            garbage_at_house.len() as i32 + travel_distance * unique_garbage_types.len() as i32;
    }

    total_cost
}

fn main() {
    let costs = vec![1, 3, 2, 5, 4, 6];
    let k = 2;
    let candidates = 2;
    let ans = total_cost(costs, k, candidates);
    println!("{}", ans);
}
