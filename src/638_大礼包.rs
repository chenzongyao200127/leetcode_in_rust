// 638_大礼包
// https://leetcode.cn/problems/shopping-offers/

use std::collections::HashMap;

impl Solution {
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
}