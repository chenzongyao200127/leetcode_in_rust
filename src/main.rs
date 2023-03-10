use std::collections::HashMap;

pub fn main() {
    let ans = min_subarray(vec![8,32,31,18,34,20,21,13,1,27,23,22,11,15,30,4,2], 148);
    assert_eq!(7, ans);
}

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut total = 0;
    for num in nums.iter() {
        total = (total + (num.rem_euclid(p))).rem_euclid(p);
    }           
    if total == 0 {
        return 0;
    }
    let mut map: HashMap<i32, i32> = HashMap::new(); //用哈希表记录前缀和对应的最大下标
    map.insert(0, -1);
    let mut pre_sum = 0;
    let mut ans = nums.len() as i32;
    for i in 0..nums.len() {
        pre_sum = (pre_sum + (nums[i].rem_euclid(p))).rem_euclid(p);
        let target = (pre_sum - total.rem_euclid(p)).rem_euclid(p);
        if let Some(j) = map.get(&target) {
            ans = ans.min((i - *j as usize) as i32);
        }
        map.insert(pre_sum, i as i32);
    }

    if ans == nums.len() as i32 {
        -1
    } else {
        ans
    }
}




// pub fn remove_kdigits(num: String, k: i32) -> String {
//     if k as usize == num.len() {
//         return "0".to_string();
//     }
//     let mut s: Vec<char> = num.chars().collect();
//     for _ in 0..k {
//         for i in 1..s.len() {
//             let pre = s[i-1] as u8;
//             if pre <= s[i] as u8 {
//                 if i == s.len()-1 {
//                     s.remove(i);
//                 } else {
//                     continue;
//                 }
//             } else {
//                 s.remove(i-1);
//                 break;
//             }
//         }
//     }

//     let mut ans: String = s.iter().collect();
//     if ans == "0".repeat(num.len() - k as usize) {
//         return "0".to_string();
//     }

// }


// class Solution:
//     def minSubarray(self, nums: List[int], p: int) -> int:
//         x = sum(nums) % p
//         if x == 0:
//             return 0
//         y = 0
//         index = {0: -1}
//         ans = len(nums)
//         for i, v in enumerate(nums):
//             y = (y + v) % p
//             if (y - x) % p in index:
//                 ans = min(ans, i - index[(y - x) % p])
//             index[y] = i
//         return ans if ans < len(nums) else -1

// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/make-sum-divisible-by-p/solution/shi-shu-zu-he-neng-bei-p-zheng-chu-by-le-dob9/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。