// 659_Split_Array_into_Consecutive_Subsequences
// https://leetcode.cn/problems/split-array-into-consecutive-subsequences/


// 659. 分割数组为连续子序列
// 给你一个按 非递减顺序 排列的整数数组 nums 。

// 请你判断是否能在将 nums 分割成 一个或多个子序列 的同时满足下述 两个 条件：

// 每个子序列都是一个 连续递增序列（即，每个整数 恰好 比前一个整数大 1 ）。
// 所有子序列的长度 至少 为 3 。
// 如果可以分割 nums 并满足上述条件，则返回 true ；否则，返回 false 。
use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut cnt_map: HashMap<i32, usize> = HashMap::new();
        let mut end_map: HashMap<i32, usize> = HashMap::new();
        for num in nums.iter() {
            cnt_map.entry(*num).and_modify(|cnt| *cnt += 1).or_insert(1);
            end_map.insert(*num, 0);
        }

        for num in nums.iter() {
            if *cnt_map.get(num).unwrap() > 0 {
                if end_map.get(&(num - 1)) != None && *end_map.get(&(num-1)).unwrap() > 0 {
                    *cnt_map.get_mut(num).unwrap() -= 1;
                    *end_map.get_mut(&(num-1)).unwrap() -= 1;
                    *end_map.get_mut(&(num)).unwrap() += 1;
                } else if cnt_map.get(&(num + 1)) != None && cnt_map.get(&(num + 2)) != None 
                    && *cnt_map.get(&(num + 1)).unwrap() > 0 && *cnt_map.get(&(num + 2)).unwrap() > 0 {
                    *cnt_map.get_mut(num).unwrap() -= 1;
                    *cnt_map.get_mut(&(num+1)).unwrap() -= 1;
                    *cnt_map.get_mut(&(num+2)).unwrap() -= 1;
                    *end_map.get_mut(&(num+2)).unwrap() += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}


impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut vecs: Vec<Vec<i32>> = Vec::new();
        for num in nums {
            let mut found = false;
            let mut found_index = 0usize;

            for vecs_index in 0..vecs.len() {
                if let Some(s) = vecs[vecs_index].last() {
                    if s + 1 == num {
                        found = true;
                        found_index = vecs_index;
                    }
                }
            }

            if found {
                vecs[found_index].push(num);
            } else {
                vecs.push(vec![num]);
            }
        }

        vecs.iter().all(|x| x.len() >= 3)
    }
}
// 作者：fh0
// 链接：https://leetcode.cn/problems/split-array-into-consecutive-subsequences/solution/rust-shuang-bai-by-fh0-z34t/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


// class Solution:
//     def isPossible(self, nums: List[int]) -> bool:
//         countMap = collections.Counter(nums)
//         endMap = collections.Counter()

//         for x in nums:
//             if (count := countMap[x]) > 0:
//                 if (prevEndCount := endMap.get(x - 1, 0)) > 0:
//                     countMap[x] -= 1
//                     endMap[x - 1] = prevEndCount - 1
//                     endMap[x] += 1
//                 else:
//                     if (count1 := countMap.get(x + 1, 0)) > 0 and (count2 := countMap.get(x + 2, 0)) > 0:
//                         countMap[x] -= 1
//                         countMap[x + 1] -= 1
//                         countMap[x + 2] -= 1
//                         endMap[x + 2] += 1
//                     else:
//                         return False
        
//         return True