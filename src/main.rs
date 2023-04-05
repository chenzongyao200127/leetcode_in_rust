fn main() {
    // let s1 = "01000111";
    // let s2 = "00111";
    // let s3 = "111";

    // println!("Example 1: {}", find_the_longest_balanced_substring(s1.to_string())); // Output: 6
    // println!("Example 2: {}", find_the_longest_balanced_substring(s2.to_string())); // Output: 4
    // println!("Example 3: {}", find_the_longest_balanced_substring(s3.to_string())); // Output: 0

    let ans = merge_stones(vec![3,5,1,2,6], 3);
    assert_eq!(ans, 25);

    let ans = merge_stones(vec![3,2,4,1], 3);
    assert_eq!(ans, -1);

    let ans = merge_stones(vec![3,2,4,1], 2);
    assert_eq!(ans, 20);
}

pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    let len = stones.len();
    if (len as i32 - 1) % (k - 1) != 0 {
        return -1;
    }
    let mut cost = 0;

    merge(stones, k, &mut cost);

    return cost;
}

pub fn merge(stones: Vec<i32>, k: i32, cost: &mut i32) -> i32 {
    if stones.len() == 1 {
        return stones[0];
    }

    let mut tmp_min = i32::MAX;
    let mut start_idx = 0;

    for i in 0..=stones.len() - k as usize {
        let mut sum = 0;
        for j in i..i+k as usize {
            sum += stones[j];
        }
        // println!("sum: {:?}", sum);
        if sum < tmp_min {
            tmp_min = sum;
            start_idx = i;
        }
    }

    let mut new_stones = vec![];

    for i in 0..start_idx {
        new_stones.push(stones[i]);
    }

    new_stones.push(tmp_min);

    for i in start_idx+k as usize..stones.len() {
        new_stones.push(stones[i]);
    }

    // println!("{:?}", (cost.clone(), tmp_min.clone()));

    *cost += tmp_min;
    
    // println!("new_stones:{:?}", new_stones);

    return merge(new_stones, k, cost);
}

pub fn common_factors(a: i32, b: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=a.min(b) as usize {
        if a as usize % i == 0 && b as usize % i == 0 {
            ans += 1;
        }
    }
    
    ans
}