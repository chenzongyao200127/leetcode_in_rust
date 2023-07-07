// 2532_Time_to_Cross_a_Bridge
// https://leetcode.cn/problems/time-to-cross-a-bridge/

// There are k workers who want to move n boxes from an old warehouse to a new one. You are given the two integers n and k, and a 2D integer array time of size k x 4 where time[i] = [leftToRighti, pickOldi, rightToLefti, putNewi].

// The warehouses are separated by a river and connected by a bridge. The old warehouse is on the right bank of the river, and the new warehouse is on the left bank of the river. Initially, all k workers are waiting on the left side of the bridge. To move the boxes, the ith worker (0-indexed) can :

// Cross the bridge from the left bank (new warehouse) to the right bank (old warehouse) in leftToRighti minutes.
// Pick a box from the old warehouse and return to the bridge in pickOldi minutes. Different workers can pick up their boxes simultaneously.
// Cross the bridge from the right bank (old warehouse) to the left bank (new warehouse) in rightToLefti minutes.
// Put the box in the new warehouse and return to the bridge in putNewi minutes. Different workers can put their boxes simultaneously.

// A worker i is less efficient than a worker j if either condition is met:
// leftToRighti + rightToLefti > leftToRightj + rightToLeftj
// leftToRighti + rightToLefti == leftToRightj + rightToLeftj and i > j

// The following rules regulate the movement of the workers through the bridge :
// If a worker x reaches the bridge while another worker y is crossing the bridge, x waits at their side of the bridge.
// If the bridge is free, the worker waiting on the right side of the bridge gets to cross the bridge. If more than one worker is waiting on the right side, the one with the lowest efficiency crosses first.
// If the bridge is free and no worker is waiting on the right side, and at least one box remains at the old warehouse, the worker on the left side of the river gets to cross the bridge. If more than one worker is waiting on the left side, the one with the lowest efficiency crosses first.
// Return the instance of time at which the last worker reaches the left bank of the river after all n boxes have been put in the new warehouse.

// 1 <= n, k <= 104
// time.length == k
// time[i].length == 4
// 1 <= leftToRighti, pickOldi, rightToLefti, putNewi <= 1000

// We can use a priority queue to query over the least efficient worker.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut time: Vec<_> = time.into_iter().enumerate().map(|(i, mut t)| {
            t.push(i as i32);
            t
        }).collect();
        time.sort_by_key(|x| (Reverse(x[0] + x[2]), Reverse(x[4])));

        let mut current_time = 0;
        let mut left_side_waiting = BinaryHeap::new();
        let mut right_side_waiting = BinaryHeap::new();
        let mut left_side_heap: BinaryHeap<_> = (0..k).map(|j| (Reverse(0), j)).collect();
        let mut right_side_heap = BinaryHeap::new();
        let mut boxes_left = n;
        let mut boxes_moved = 0;

        while boxes_left > 0 {
            if let Some((_, worker_idx)) = right_side_heap.pop() {
                left_side_waiting.push((Reverse(current_time + time[worker_idx as usize][2] + time[worker_idx as usize][3]), worker_idx));
                current_time += time[worker_idx as usize][2];
                boxes_left -= 1;
            } else if boxes_moved < n {
                let (_, worker_idx) = left_side_heap.pop().unwrap();
                right_side_waiting.push((Reverse(current_time + time[worker_idx as usize][0] + time[worker_idx as usize][1]), worker_idx));
                current_time += time[worker_idx as usize][0];
                boxes_moved += 1;
            } else {
                current_time = *[
                    left_side_waiting.peek().map(|&(Reverse(t), _)| t).unwrap_or(10_i32.pow(9)),
                    right_side_waiting.peek().map(|&(Reverse(t), _)| t).unwrap_or(10_i32.pow(9)),
                ].iter().min().unwrap();
            }

            while right_side_waiting.peek().map_or(false, |&(Reverse(t), _)| t <= current_time) {
                let (_, worker_idx) = right_side_waiting.pop().unwrap();
                right_side_heap.push((Reverse(worker_idx), current_time));
            }

            while left_side_waiting.peek().map_or(false, |&(Reverse(t), _)| t <= current_time) {
                let (_, worker_idx) = left_side_waiting.pop().unwrap();
                left_side_heap.push((Reverse(worker_idx), current_time));
            }
        }

        current_time
    }
}

// Struct std::cmp::ReverseCopy 

#[repr(transparent)]
pub struct Reverse<T>(pub T);
// 用于逆序排序的辅助结构体。

// 该结构是一个帮助器，可与 Vec::sort_by_key 等函数一起使用，并且可以用于对键的一部分进行反向排序。


// use std::cmp::Reverse;

// let mut v = vec![1, 2, 3, 4, 5, 6];
// v.sort_by_key(|&num| (num > 3, Reverse(num)));
// assert_eq!(v, vec![3, 2, 1, 6, 5, 4]);

/// Rust does not include a built-in priority queue, 
/// but it has a BinaryHeap type in the standard library which can be used as a max priority queue. 
/// If you want to use it as a min priority queue, you need to wrap your values into std::cmp::Reverse.

// Here is an example of using BinaryHeap:
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(3);
    heap.push(5);
    heap.push(1);

    // Heap is a max heap, so the elements will be popped in descending order
    while let Some(val) = heap.pop() {
        println!("{}", val);
    }
}

/// In this code:

/// We create a BinaryHeap with BinaryHeap::new.
/// We push some values onto the heap with heap.push.
/// We pop all values from the heap with heap.pop in a loop, 
/// which returns Some(val) until the heap is empty, at which point it returns None and the loop stops.
/// This code will print out:

// 5
// 3
// 1

/// If you want to implement a min-heap (smallest value has the highest priority), you can use std::cmp::Reverse:

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(3));
    heap.push(Reverse(5));
    heap.push(Reverse(1));

    // Heap is now a min heap because of the Reverse, so the elements will be popped in ascending order
    while let Some(Reverse(val)) = heap.pop() {
        println!("{}", val);
    }
}

// This code will print out:

// 1
// 3
// 5

/// In this code, Reverse is a newtype that changes the sort order by reversing the comparison results.

/// If you need more complex priority queue functionality (for example, 
/// a priority queue that supports updating the priority of items), 
/// you may need to use a third-party crate such as priority-queue or heapless.