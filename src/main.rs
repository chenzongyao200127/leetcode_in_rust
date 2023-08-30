

pub fn num_distinct(s: String, t: String) -> i32 {
    let s: Vec<_> = s.chars().collect();
    let t: Vec<_> = t.chars().collect();

    let mut dp = vec![vec![0; s.len()+1]; t.len()+1];

    for i in 0..s.len() {
        if s[i] == t[0] {
            dp[1][i+1] = dp[1][i] + 1;
        } else {
            dp[1][i+1] = dp[1][i];
        }
    }

    for i in 1..t.len() {
        for j in 0..s.len() {
            if s[j] == t[i] {
                dp[i+1][j+1] = dp[i+1][j] + dp[i][j];
            } else {
                dp[i+1][j+1] = dp[i+1][j]
            }
        }
    }

    println!("{:?}", dp);

    dp[t.len()][s.len()]
}




pub fn min_cut(s: String) -> i32 {
    let mut s1 = s.clone();
    let mut s2: String;
    let mut cnt = 0;

    loop {
        s2 = s1.chars().rev().collect();
        match longest_common_substring(&s1, &s2) {
            Some(common) => {
                s1 = remove_substring_from(&s1, common);
                cnt += 1;
            },
            None => break,
        }
    }

    cnt
}

pub fn longest_common_substring<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let mut longest_len = 0;
    let mut longest_pos = 0;

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1_bytes[i-1] == s2_bytes[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
                if dp[i][j] > longest_len {
                    longest_len = dp[i][j];
                    longest_pos = i - longest_len;
                }
            }
        }
    }

    if longest_len == 0 {
        None
    } else {
        Some(&s1[longest_pos..longest_pos + longest_len])
    }
}

pub fn remove_substring_from(s: &str, sub: &str) -> String {
    s.replace(sub, "")
}


use std::collections::HashMap;

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
        return false;
    }

    if desired_total <= max_choosable_integer {
        return true;
    }

    fn dfs(
        state: i32, 
        cur_total: i32, 
        max_choosable_integer: i32, 
        desired_total: i32, 
        memo: &mut HashMap<i32, bool>
    ) -> bool {
        if let Some(&v) = memo.get(&state) {
            return v;
        }

        for i in (1..=max_choosable_integer).rev() {
            if (state & (1 << (i - 1))) == 0 {
                if cur_total + i >= desired_total 
                    || !dfs(
                        state | (1 << (i - 1)), 
                        cur_total + i, 
                        max_choosable_integer, 
                        desired_total, 
                        memo
                    ) {
                    memo.insert(state, true);
                    return true;
                }
            }
        }

        memo.insert(state, false);
        return false;
    }

    let mut memo = HashMap::new();
    dfs(0, 0, max_choosable_integer, desired_total, &mut memo)
}



pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len();
    let mut l: Option<usize> = None; // Initialize l as None.
    let mut max_distance = 0;

    for i in 0..n {
        if seats[i] == 1 {
            if l.is_none() { // Check if l is None.
                max_distance = i as i32;
            } else {
                // Unwrap safely because we checked with is_none().
                max_distance = max_distance.max((i - l.unwrap()) as i32 / 2);
            }
            l = Some(i);
        }
    }

    // For the case where the last seated person is not at the end.
    if seats[n-1] == 0 {
        max_distance = max_distance.max((n - 1 - l.unwrap()) as i32);
    }

    max_distance
}


struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        if sum < target as i64 {
            return -1;
        }

        let mut count = HashMap::new();
        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut operations: i64 = 0;
        let mut total_sum: i64 = 0;

        for i in 0..31 {
            total_sum += *count.get(&(1 << i)).unwrap_or(&0) as i64 * (1 << i) as i64;

            if (target >> i & 1) == 0 {
                continue;
            }

            total_sum -= (1 << i) as i64;

            if total_sum >= 0 {
                continue;
            }

            for j in (i + 1)..31 {
                if let Some(cnt) = count.get_mut(&(1 << j)) {
                    if *cnt > 0 {
                        operations += j as i64 - i as i64;
                        *cnt -= 1;
                        total_sum += (1 << j) as i64;
                        break;
                    }
                }
            }
        }

        operations as i32
    }
}


pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let states = 1 << n;
    let mut dp = vec![i32::MAX; states as usize];
    let target_ch: Vec<char> = target.chars().collect();
    dp[0] = 0;

    for s in 0..states as usize {
        if dp[s] == i32::MAX {
            continue;
        }

        for strick in stickers.iter() {
            let mut nxt = s;
            for ch in strick.chars() {
                for k in 0..n {
                    if ch == target_ch[k] && s & (1 << k) == 0 {
                        nxt |= 1 << k;
                        println!("{:?}", nxt);
                        break;
                    }
                }
            }

            dp[nxt] = dp[nxt].min(dp[s] + 1);
        }
    }


    if dp[dp.len() - 1] == i32::MAX {
        -1
    } else {
        dp[dp.len() - 1]
    }
}


pub fn num_factored_binary_trees(arr: Vec<i32>) -> i64 {
    const MOD: i32 = 1_000_000_007;
    
    let mut arr = arr.clone();
    arr.sort();
    
    let index: HashMap<i32, usize> = arr.iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();
    
    let mut dp = vec![1; arr.len()];
    
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                let right_child = arr[i] / arr[j];
                if let Some(&right_index) = index.get(&right_child) {
                    dp[i] = (dp[i] + dp[j] as i64 * dp[right_index] as i64 % MOD as i64) % MOD as i64;
                }
            }
        }
    }
    
    dp.iter().sum::<i64>() % MOD as i64
}

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let total: i32 = nums.iter().sum();
    if total % k != 0 {
        return false;
    }

    let target = total / k;
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a));
    
    if nums[0] > target {
        return false;
    }

    let mut visited = vec![false; nums.len()];
    can_partition(&nums, &mut visited, 0, k, 0, target)
}

fn can_partition(nums: &Vec<i32>, visited: &mut Vec<bool>, start: usize, k: i32, curr_sum: i32, target: i32) -> bool {
    if k == 1 {
        return true;
    }

    if curr_sum == target {
        return  can_partition(nums, visited, 0, k - 1, 0, target);
    }
    
    for i in start..nums.len() {
        if !visited[i] && curr_sum + nums[i] <= target {
            visited[i] = true;
            if  can_partition(nums, visited, i + 1, k, curr_sum + nums[i], target) {
                return true;
            }
            visited[i] = false;
        }
    }
    false
}

fn main() {
    let nums1 = vec![4, 3, 2, 3, 5, 2, 1];
    let k1 = 4;
    println!("{}", can_partition_k_subsets(nums1, k1)); // True

    let nums2 = vec![1, 2, 3, 4];
    let k2 = 3;
    println!("{}", can_partition_k_subsets(nums2, k2)); // False
}
