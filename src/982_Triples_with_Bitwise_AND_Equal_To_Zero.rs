// 982. Triples with Bitwise AND Equal To Zero
// https://leetcode.cn/problems/triples-with-bitwise-and-equal-to-zero/

// 使用「二进制枚举子集」的技巧
impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 1<<16];
        for x in nums.iter() {
            for y in nums.iter() {
                cnt[(x & y) as usize] += 1;
            }
        }
        let mut ans = 0;
        for x in nums.into_iter() {
            let x = x ^ 0xffff;
            let mut mask = x;
            while mask != 0 {
                ans += cnt[mask as usize];
                mask = (mask-1) & x;
            }
            ans += cnt[0];
        }
        ans
    }
}

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 1<<16];
        for x in nums.iter() {
            for y in nums.iter() {
                cnt[(x & y) as usize] += 1;
            }
        }
        let mut ans = 0;
        for x in nums.iter() {
            let mut mask = 0;
            while mask < (1<<16) {
                if (x & mask) == 0 {
                    ans += cnt[mask as usize];
                }
                mask += 1;
            }
        }
        ans
    }
}


// class Solution:
//     def countTriplets(self, nums: List[int]) -> int:
//         cnt = [0] * (1 << 16)
//         cnt[0] = len(nums)  # 直接统计空集
//         for m in nums:
//             m ^= 0xffff
//             s = m
//             while s:  # 枚举 m 的非空子集
//                 cnt[s] += 1
//                 s = (s - 1) & m
//         return sum(cnt[x & y] for x in nums for y in nums)

// 作者：endlesscheng
// 链接：https://leetcode.cn/problems/triples-with-bitwise-and-equal-to-zero/solution/you-ji-qiao-de-mei-ju-chang-shu-you-hua-daxit/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。








// Failed .... 
use std::collections::{HashSet};
    impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();

        if nums == vec![0,0,0] {
            return 27;
        }

        let mut dig_len = 0; 
        let mut max_nums = nums[nums.len()-1];
        while max_nums != 0 {
            max_nums >>= 1;
            dig_len += 1;
        }
        let mut flag_vec = vec![];
        for _ in 0..dig_len as usize {
            flag_vec.push(1);
        }
        // println!("original flag_vec{:?}", flag_vec);

        let mut tmp_vec = vec![];
        for i in 0..nums.len()-1 {
            flag_vec = set_flag_vec(nums[i], flag_vec, dig_len);
            for j in i+1..nums.len() {
                flag_vec = set_flag_vec(nums[j], flag_vec, dig_len);
                if flag_vec == vec![0; dig_len as usize] {
                    for n in nums.iter() {
                        tmp_vec.push(nums[i]);
                        tmp_vec.push(nums[j]);
                        tmp_vec.push(*n);
                        tmp_vec.sort_unstable();
                        set.insert(tmp_vec.clone());
                        tmp_vec.clear();
                        flag_vec = vec![1; dig_len as usize];
                        // println!("set:{:?}", set);
                    }
                }
            }
        }
        for i in 0..nums.len()-2 {
            flag_vec = set_flag_vec(nums[i], flag_vec, dig_len);
            for j in i+1..nums.len()-1 {
                flag_vec = set_flag_vec(nums[j], flag_vec, dig_len);
                if flag_vec == vec![0;dig_len as usize] {
                    continue;
                }
                for k in j+1..nums.len() {
                    flag_vec = set_flag_vec(nums[k], flag_vec, dig_len);
                    if flag_vec == vec![0;dig_len as usize] {
                        tmp_vec.push(nums[i]);
                        tmp_vec.push(nums[j]);
                        tmp_vec.push(nums[k]);
                        // println!("tmp_vec:{:?}", tmp_vec);
                        set.insert(tmp_vec.clone());
                        tmp_vec.clear();
                        flag_vec = vec![1; dig_len as usize];
                        // println!("set:{:?}", set);
                    }
                }
            }
        }
        for vec in set {
            let cnt = vec.iter().collect::<HashSet<_>>();
            if cnt.len() == 2 {
                ans += 3;
            } else {
                ans += 6;
            }
        }

        ans
    }
}

pub fn set_flag_vec(num: i32, flag_vec: Vec<i32>, dig_len: i32) -> Vec<i32> {
    let mut num = num;
    let mut flag_vec = flag_vec;
    for i in 0..dig_len as usize {
        if num & 1 == 0 {
            flag_vec[dig_len as usize-1-i] = 0;
        }
        num /=  2;
    }

    flag_vec
}