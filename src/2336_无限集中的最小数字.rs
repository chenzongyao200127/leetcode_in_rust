// 2336_无限集中的最小数字
// https://leetcode.cn/problems/smallest-number-in-infinite-set/description/

// 现有一个包含所有正整数的集合 [1, 2, 3, 4, 5, ...] 。
// 实现 SmallestInfiniteSet 类：
// SmallestInfiniteSet() 初始化 SmallestInfiniteSet 对象以包含 所有 正整数。
// int popSmallest() 移除 并返回该无限集中的最小整数。
// void addBack(int num) 如果正整数 num 不 存在于无限集中，则将一个 num 添加 到该无限集中。

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    start: i32,
    pq: BinaryHeap<Reverse<i32>>,
    pq_set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// 引入标准库中的比较和集合模块。
use std::cmp::Reverse; // Reverse 用于反转排序顺序。
use std::collections::{BinaryHeap, HashSet}; // BinaryHeap 是一个标准的二叉堆实现，HashSet 是一个哈希集合。

// 定义一个结构体 SmallestInfiniteSet。
struct SmallestInfiniteSet {
    start: i32, // start 字段，用来记录下一个自然增长的整数。
    pq: BinaryHeap<Reverse<i32>>, // 一个存储整数的最小堆（通过 Reverse 实现）。
    pq_set: HashSet<i32>, // HashSet 用于跟踪已经被加回堆中的数字。
}

impl SmallestInfiniteSet {
    // 构造函数，用于初始化 SmallestInfiniteSet。
    fn new() -> Self {
        Self {
            start: 1, // 初始化 start 为 1。
            pq: BinaryHeap::new(), // 创建一个新的空最小堆。
            pq_set: HashSet::new(), // 创建一个新的空 HashSet。
        }
    }

    // 从集合中弹出最小的元素。
    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(v)) = self.pq.pop() {
            // 如果堆不为空，则弹出并返回最小元素。
            self.pq_set.remove(&v); // 从 HashSet 中移除这个元素。
            return v;
        } else {
            // 如果堆为空，则返回 start 并递增。
            self.start += 1;
            return self.start - 1;
        }
    }

    // 将一个数字加回集合中。
    fn add_back(&mut self, num: i32) {
        if self.start > num && self.pq_set.insert(num) {
            // 如果数字小于 start 并且未在 HashSet 中，则加入堆。
            self.pq.push(Reverse(num)); // 将数字加入最小堆。
        }
    }
}


/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

 