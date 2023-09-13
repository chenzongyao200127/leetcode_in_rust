use std::{collections::HashMap};


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

fn main() {
    assert_eq!(makesquare(vec![5,5,5,5,4,4,4,4,3,3,3,3]), true)
}
