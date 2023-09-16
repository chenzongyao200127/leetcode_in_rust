use std::clone;
use std::{collections::HashMap, vec};


pub fn shopping_offers(price: Vec<i32>, mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let l = price.len();

    // Filter out offers which are not cost-effective
    special.retain(|s| s.last().unwrap() < &s.iter().take(l).enumerate().map(|(i, &val)| val * price[i]).sum::<i32>());

    let mut memo = std::collections::HashMap::new();

    #[inline]
    fn shopping(remaining_needs: Vec<i32>, n: usize, price: &[i32], special: &[Vec<i32>], memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        if remaining_needs.iter().sum::<i32>() == 0 {
            return 0;
        }

        if let Some(&cached) = memo.get(&remaining_needs) {
            return cached;
        }

        // Filter out offers which exceed current needs
        let valid_offers: Vec<&Vec<i32>> = special.iter().filter(|&offer| {
            offer.iter().take(n).zip(&remaining_needs).all(|(&offer_val, &need_val)| offer_val <= need_val)
        }).collect();

        let cost = if valid_offers.is_empty() {
            remaining_needs.iter().enumerate().map(|(i, &val)| val * price[i]).sum::<i32>()
        } else {
            let mut costs_with_offers = vec![];
            for offer in &valid_offers {
                let cost_with_offer = offer[n] + shopping(
                    remaining_needs.iter().enumerate().map(|(i, &val)| val - offer[i]).collect(),
                    n,
                    &price,
                    &special,
                    memo
                );
                costs_with_offers.push(cost_with_offer);
            }
            *costs_with_offers.iter().min().unwrap()
        };

        memo.insert(remaining_needs, cost);

        cost
    }

    shopping(needs, l, &price, &special, &mut memo)
}

pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
        return false;
    }

    #[inline]
    fn dfs(x: usize, y: usize, g: &Vec<Vec<i32>>, cur_value: i32) -> bool {
        if cur_value == (g.len() * g.len() - 1) as i32 {
            return true;
        }

        let dirs = vec![(-1, -2), (-2, -1), (-1, 2), (-2, 1), (1, 2), (2, 1), (1, -2), (2, -1)];
        let mut flag = false;
        for dir in dirs {
            let (dx, dy) = dir;
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < g.len() as i32 && new_y >= 0 && new_y < g.len() as i32 {
                if g[new_x as usize][new_y as usize] == cur_value + 1 {
                    flag |= dfs(new_x as usize, new_y as usize, g, cur_value + 1);
                }
            }
        }

        flag
    }

    dfs(0, 0, &grid, 0)
}

pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
    let n = matchsticks.len();

    matchsticks.sort_unstable();

    let total = matchsticks.iter().sum::<i32>();
    
    if total % 4 != 0 {
        return false;
    }

    let target = total / 4;

    let mut dp = vec![-1; 1 << n];

    dp[0] = 0;

    for mask in 0..1 << n {
        if dp[mask as usize] == -1 {
            continue;
        }

        for j in 0..n {
            let new_mask = mask | (1 << j);
            if mask & (1 << n) != 1 && dp[mask] + matchsticks[j] <= target {
                dp[new_mask] = (dp[mask] + matchsticks[j]) % target;
            }

        }
    }

    println!("{:?}", dp);
    dp[(1 << n) - 1] == 0
}



pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();

    #[inline]
    fn dfs(l: usize, r: usize, nums: &Vec<i32>, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
        if l == r {
            return nums[l];
        }
        
        if let Some(&res) = memo.get(&(l, r)) {
            return res;
        }
        
        let pick_first = nums[l] - dfs(l + 1, r, nums, memo);
        let pick_last = nums[r] - dfs(l, r - 1, nums, memo);

        let res = pick_first.max(pick_last);
        
        memo.insert((l, r), res);
        res
    }

    dfs(0, nums.len()-1, &nums, &mut memo) >= 0
}

// Input: queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]], king = [0,0]
// Output: [[0,1],[1,0],[3,3]]

// Input: queens = [[0,0],[1,1],[2,2],[3,4],[3,5],[4,4],[4,5]], king = [3,3]
// Output: [[2,2],[3,4],[4,4]]

use std::collections::HashSet;

use rand::distributions::Bernoulli;

pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    #[inline]
    fn is_valid(x: i32, y: i32) -> bool {
        x < 8 && y < 8 && x >= 0 && y >= 0
    }

    let mut ans = vec![];

    let x_king = king[0];
    let y_king = king[1];

    let queen_set: HashSet<(i32, i32)> = queens.into_iter().map(|q| (q[0], q[1])).collect();
     
    let dirs: [(i32, i32); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
    
    for (dx, dy) in dirs.iter() {
        let mut new_x = x_king + dx;
        let mut new_y = y_king + dy;
        while is_valid(new_x, new_y) {
            if queen_set.contains(&(new_x, new_y)) {
                ans.push(vec![new_x, new_y]);
                break;
            }
            new_x += dx;
            new_y += dy;
        }
    }

    ans
}



pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans = vec![-1, -1];

    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut mid = l + (r - l) / 2;

    if nums.is_empty() {
        return ans;
    }

    if nums[0] > target {
        return ans;
    }

    if nums[r] < target {
        return ans;
    }
    
    while l <= r {
        if nums[mid] == target {
            let mut tmp = mid as i32;
            while tmp >= 0 && nums[tmp as usize] == target {
                tmp -= 1;
            }
            ans[0] = tmp as i32 + 1;

            tmp = mid as i32;
            while tmp < nums.len() as i32 - 1 && nums[tmp as usize] == target {
                tmp += 1;
            }
            if tmp as usize == mid {
                ans[1] = tmp as i32;
            } else {
                if tmp == nums.len() as i32 - 1 && nums[tmp as usize] == target {
                    ans[1] = tmp as i32;
                } else {
                    ans[1] = tmp as i32 - 1;
                }
            }
            break;
        } else if nums[mid] < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }

        if r < l {
            break;
        }
        mid = l + (r - l) / 2;
    }

    ans
}

pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;
    let mut mid = l + (r - l) / 2;
    while l <= r {
        if check(mid, &nums) {
            return mid;
        } else {
            // let a1 = single_non_duplicate(nums[:l]);
            // let a2 = single_non_duplicate(nums[r:]);
            // return a1.max(a2);
        }
    }
    return -1;
}

pub fn check(idx: i32, nums: &Vec<i32>) -> bool {
    if idx - 1 >= 0 {
        if nums[idx as usize] == nums[idx as usize - 1] {
            return true
        } 
    }
    if idx + 1 <= nums.len() as i32 - 1 {
        if nums[idx as usize] == nums[idx as usize + 1] {
            return true;
        }
    }
    false
}


fn main() {
    let ans = search_range(vec![2, 2], 2);
    println!("{:?}", ans);
}
