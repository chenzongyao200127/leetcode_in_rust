// 374_Guess_Number_Higher_or_Lower
// https://leetcode.cn/problems/guess-number-higher-or-lower/

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let left = 1;
        let right = n;
        return guess_num(left, right);
    }
}

pub unsafe fn guess_num(left: i32, right: i32) -> i32 {
    let mid = left + ((right - left) >> 1);
    match guess(mid) {
        0 => {return mid;},
        -1 => {return guess_num(left, mid-1);},
        _ => {return guess_num(mid+1, right)}
    }
}

// pub unsafe fn guess_num(left: i32, right: i32) -> i32 {
//     let mid = (left + right + 1) / 2; // 这样写就会超时，原因是什么？
//     match guess(mid) {
//         0 => {return mid;},
//         -1 => {return guess_num(left, mid-1);},
//         _ => {return guess_num(mid+1, right)}
//     }
// }


// class Solution:
//     def guessNumber(self, n: int) -> int:
//         def bsearch(left,right):
//             mid = left + ((right - left) >> 1)
//             if guess(mid) == 0:
//                 return mid
//             elif guess(mid) == -1:
//                 return bsearch(left, mid - 1)
//             else: 
//                 return bsearch(mid + 1, right)
//         return bsearch(1,n)


/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */



impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        while left < right {  // 循环直至区间左右端点相同
            let m = left + (right - left) / 2; // 防止计算时溢出
            if guess(m) == 1 {
                left = m + 1;
            } else {
                right = m
            }
        }
        left  // 此时有 left == right，区间缩为一个点，即为答案
    }
}

