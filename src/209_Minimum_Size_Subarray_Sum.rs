// 209_Minimum_Size_Subarray_Sum
// https://leetcode.cn/problems/minimum-size-subarray-sum/

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < target {
            return 0;
        }

        let mut start = 0 as usize;
        let mut end = 0 as usize;
        let mut tmp_sum = nums[start];
        while tmp_sum < target {
            end += 1;
            tmp_sum += nums[end];
        }
        let mut ans = end - start + 1;

        while start < nums.len() {
            start += 1;
            tmp_sum -= nums[start-1];
            while tmp_sum < target && end < nums.len()-1 {
                end += 1;
                tmp_sum += nums[end];
            }
            if tmp_sum >= target {
                ans = ans.min(end - start + 1);
            }
            if ans == 1 {
                return 1;
            }
        }

        ans as i32
    }
}


/// 示例代码
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut ans = nums.len() + 1;
        let mut acc = 0;
        
        while right < nums.len() {
            while right < nums.len() && acc < target {
                acc += nums[right];
                right += 1;
            }
            while acc >= target {
                ans = ans.min(right - left);
                acc -= nums[left];
                left += 1;
            }
        }
        if ans == nums.len() + 1 { return 0; } 
        return ans as i32;
    }
}

/// 方法二：前缀和 + 二分查找
/// 因为这道题保证了数组中每个元素都为正，所以前缀和一定是递增的，这一点保证了二分的正确性。
/// 如果题目没有说明数组中每个元素都为正，这里就不能使用二分来查找这个位置了。
///
// class Solution:
//     def minSubArrayLen(self, s: int, nums: List[int]) -> int:
//         if not nums:
//             return 0
        
//         n = len(nums)
//         ans = n + 1
//         sums = [0]
//         for i in range(n):
//             sums.append(sums[-1] + nums[i])
        
//         for i in range(1, n + 1):
//             target = s + sums[i - 1]
//             bound = bisect.bisect_left(sums, target)
//             if bound != len(sums):
//                 ans = min(ans, bound - (i - 1))
        
//         return 0 if ans == n + 1 else ans

// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/minimum-size-subarray-sum/solution/chang-du-zui-xiao-de-zi-shu-zu-by-leetcode-solutio/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。