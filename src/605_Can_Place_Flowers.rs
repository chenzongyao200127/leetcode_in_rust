// 605. Can Place Flowers
// https://leetcode.cn/problems/can-place-flowers/

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut new_flowerbed = vec![0];
        flowerbed.iter().for_each(|i| new_flowerbed.push(*i));
        new_flowerbed.push(0);
        let mut ans = 0;
        for i in 1..new_flowerbed.len()-1 {
            if new_flowerbed[i-1] == 0 && new_flowerbed[i] == 0 && new_flowerbed[i+1] == 0 {
                new_flowerbed[i] = 1;
                ans += 1;
            }
        }
        ans >= n
    }
}