// 135. 分发糖果
// https://leetcode.cn/problems/candy/

// https://leetcode.cn/link/?target=https://www.bilibili.com/video/BV1WK4y1R71x
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut kid_candy = vec![];
        for _ in 0..ratings.len() {
            kid_candy.push(1);
        }
        for i in 0..ratings.len()-1 {
            if ratings[i+1] > ratings[i] {
                kid_candy[i+1] = kid_candy[i] + 1;
            }
        }

        for i in (1..ratings.len()).rev() {
            if ratings[i-1] > ratings[i] && kid_candy[i-1] <= kid_candy[i]{
                kid_candy[i-1] = kid_candy[i] + 1;
            }
        }

        kid_candy.iter().sum::<i32>()
    }
}


impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        let mut candy_list_left = vec![-1; len];
        candy_list_left[0] = 1;
        for index in 1..len {
            if ratings[index] > ratings[index - 1] {
                candy_list_left[index] = candy_list_left[index - 1] + 1;
            } else {
                candy_list_left[index] = 1;
            }
        }

        let mut candy_list_right = vec![-1; len];
        candy_list_right[len - 1] = 1;
        for fake_index in 1..len {
            let index = len - fake_index - 1;
            if ratings[index] > ratings[index + 1] {
                candy_list_right[index] = candy_list_right[index + 1] + 1;
            } else {
                candy_list_right[index] = 1;
            }
        }
        let mut res = 0;
        for index in 0..len {
            res += candy_list_left[index].max(candy_list_right[index]);
        }
        res
    }
}


// class Solution:
//     def candy(self, ratings: List[int]) -> int:
//         candyVec = [1] * len(ratings)
//         for i in range(1, len(ratings)):
//             if ratings[i] > ratings[i - 1]:
//                 candyVec[i] = candyVec[i - 1] + 1
//         for j in range(len(ratings) - 2, -1, -1):
//             if ratings[j] > ratings[j + 1]:
//                 candyVec[j] = max(candyVec[j], candyVec[j + 1] + 1)
//         return sum(candyVec)