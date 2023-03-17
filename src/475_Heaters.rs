// 475_Heaters
// https://leetcode.cn/problems/heaters/

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let (mut houses, mut heaters) = (houses, heaters);
        let mut ans = 0;
        houses.sort_unstable();
        heaters.sort_unstable();
        let mut i = 0; // 很巧妙的遍历
        for &h in houses.iter() {
            let mut dis = i32::abs(heaters[i] - h);
            while i + 1 < heaters.len() && i32::abs(heaters[i + 1] - h) <= dis {
                dis = i32::abs(heaters[i + 1] - h);
                i += 1;
            }
            ans = ans.max(dis);
        }

        ans
    }
}

// class Solution:
//     def findRadius(self, houses: List[int], heaters: List[int]) -> int:
//         heaters = heaters + [-inf, inf]
//         houses.sort()
//         heaters.sort()
//         i, j, ans = 0, 0, 0
//         while i < len(houses):
//             cur = inf
//             while heaters[j] <= houses[i]:
//                 cur = houses[i] - heaters[j]
//                 j += 1
//             cur = min(cur, heaters[j] - houses[i])
//             ans = max(cur, ans)
//             i += 1
//             j -= 1
//         return ans





/// 对于每个房屋，要么用前面的暖气，要么用后面的，二者取近的，得到距离；
/// 对于所有的房屋，选择最大的上述距离。
/// 无论双指针还是二分，本质都是找当前房间最近的供暖器。如果不理解二分查出来的结果的判断，可以看下注释，结尾附上了了golang的二分查找的源码，感兴趣的同学可以看看
/// 这道题可以这样考虑，供暖器的位置是确定的，
/// 所以房间所选的供暖器的位置是从自身位置开始向两边展开，供暖器越远，供暖器需要辐射的半径就会越大。
/// 我们需要尽可能的让供暖器半径变小，所以我们应该让能覆盖当前房间的供暖器尽可能靠近房间，所以会有以下三种情况：
/// - 当前房间有供暖器，这是我们就选当前位置为0
/// 右边最近：heaters[i] - house
/// 左边最近：house - heaters[i]
/// 这时我们选取最近的供暖器就可以，我们无需关心选取的供暖器会因为后面的房间的覆盖而增大半径，
/// 因为后面的房间让半径增大后，最后的结果也会随之改变，而且之前的房间依然可以被覆盖。
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut map = vec![0; (houses[houses.len()-1] as usize + 1).max(heaters[heaters.len()-1] as usize + 1)];
        heaters.iter().for_each(|&heater| map[heater as usize] = 1);
        let mut ans = 0;
        let mut heaters = heaters;
        heaters.sort_unstable(); 
        for &house in houses.iter() {
            let mut left = 0;
            let mut right = 0;
            if house < heaters[0] {
                left = i32::MAX;
                while map[(house + right) as usize] != 1 {
                    right += 1;
                }
            } else if house > heaters[heaters.len()-1] {
                right = i32::MAX;
                while map[(house + left) as usize] != 1 {
                    left -= 1;
                }
            } else {
                while map[(house + left) as usize] != 1 && map[(house + right) as usize] != 1 {
                    left -= 1;
                    right += 1;
                }
            }
            let tmp = (left.abs()).min(right.abs());
            ans = ans.max(tmp);
        }

        ans
    }
}