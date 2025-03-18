// 2614_Prime_In_Diagonal
// https://leetcode.cn/problems/prime-in-diagonal/description/?envType=daily-question&envId=2025-03-18

// You are given a 0-indexed two-dimensional integer array nums.
// Return the largest prime number that lies on at least one of the diagonals of nums. In case, no prime is present on any of the diagonals, return 0.

// Note that:
// An integer is prime if it is greater than 1 and has no positive integer divisors other than 1 and itself.
// An integer val is on one of the diagonals of nums if there exists an integer i for which nums[i][i] = val or an i for which nums[i][nums.length - i - 1] = val.

impl Solution {
    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as i32 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        for i in 0..n {
            if Self::is_prime(nums[i][i]) {
                res = res.max(nums[i][i]);
            }
            if Self::is_prime(nums[i][n - i - 1]) {
                res = res.max(nums[i][n - i - 1]);
            }
        }
        res
    }
}
