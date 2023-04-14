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

use std::collections::HashMap;

pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let nums: Vec<i32> = nums.iter().filter(|&x| *x & 1 == 0).collect();
    if nums.len() == 0 {
        return -1;
    }
    let mut map: HashMap<i32, usize> = HashMap::new();
    for n in nums {
        map.entry(n).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    let mut res = vec![];
    for (k, v) in map {
        res.push((k, v));
    }
    res.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    res = res.iter().filter(|&(k, v)| *v == res[0].1).cloned().collect(); // Assign the filtered results back to the `res` variable
    res.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    res[0].0
}