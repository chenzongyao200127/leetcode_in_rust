use std::cell::Cell;

#[derive(Debug)]
struct Unmovable<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}

fn main() {
    let test = Unmovable { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}


// pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
//     let mut nums = nums;
//     let nums: Vec<i32> = nums.iter().filter(|&x| *x & 1 == 0).collect();
//     if nums.len() == 0 {
//         return -1;
//     }
//     let mut map: HashMap<i32, usize> = HashMap::new();
//     for n in nums {
//         map.entry(n).and_modify(|cnt| *cnt += 1).or_insert(1);
//     }
//     let mut res = vec![];
//     for (k, v) in map {
//         res.push((k, v));
//     }
//     res.sort_unstable_by(|a, b| b.1.cmp(&a.1));
//     res = res.iter().filter(|&(k, v)| *v == res[0].1).cloned().collect(); // Assign the filtered results back to the `res` variable
//     res.sort_unstable_by(|a, b| a.0.cmp(&b.0));
//     res[0].0
// }

use std::collections::HashMap;
use std::cmp::Ordering;

pub struct MajorityChecker {
    arr: Vec<i32>,
    lst: Vec<(i32, Vec<usize>)>,
}

impl MajorityChecker {
    pub fn new(arr: Vec<i32>) -> Self {
        let mut dct = HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            dct.entry(num).or_insert_with(Vec::new).push(i);
        }
        let mut lst: Vec<(i32, Vec<usize>)> = dct.into_iter().collect();
        lst.sort_unstable_by(|a, b| b.1.len().cmp(&a.1.len()));
        MajorityChecker { arr, lst }
    }

    pub fn query(&self, left: usize, right: usize, threshold: usize) -> i32 {
        if left + 1 == right {
            return if self.arr[left] != self.arr[right] { -1 } else { self.arr[left] };
        }
        if left == right {
            return self.arr[left];
        }
        for (num, ind) in self.lst.iter() {
            if ind.len() < threshold {
                break;
            }
            let cur = ind.binary_search(&right).unwrap_or_else(|x| x)
                - ind.binary_search(&left).unwrap_or_else(|x| x);
            if cur >= threshold {
                return *num;
            }
        }
        -1
    }
}