// 2517_Maximum_Tastiness_of_Candy_Basket
// https://leetcode.cn/problems/maximum-tastiness-of-candy-basket/
impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        let mut price = price;
        let len = price.len();
        price.sort_unstable();
        let mut left = 0;
        let mut right = price[len-1] - price[0];
        while left < right {
            let mid = (left + right + 1) >> 1;
            if check(&price, k, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        return left;

    }
}

pub fn check(price: &Vec<i32>, k: i32, tastiness: i32) -> bool {
    let mut prev = price[0];
    let mut cnt = 1;
    for p in price.iter().skip(1) {
        if *p - prev >= tastiness {
            cnt += 1;
            prev = *p;
        }
    }
    return cnt >= k;
}




impl Solution {
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        let check = |price: &[i32], diff: i32| -> bool {
            let mut cnt = 1;
            let mut lst = price[0];
            for &x in price.iter().skip(1) {
                if x >= lst + diff {
                    cnt += 1;
                    lst = x;
                }
            }
            cnt >= k
        };

        price.sort_unstable();
        let mut i = 0;
        let mut j = (*price.last().unwrap() - price[0]) / (k - 1) + 1;
        while i < j {
            let m = (i + j + 1) >> 1;
            if check(&price, m) {
                i = m;
            } else {
                j = m - 1;
            }
        }
        i
    }
}