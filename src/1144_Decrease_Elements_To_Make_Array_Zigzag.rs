// 1144. Decrease Elements To Make Array Zigzag
// https://leetcode.cn/problems/decrease-elements-to-make-array-zigzag/


//                 int left = i ? nums[i - 1] : INT_MAX;
//                 int right = i < n - 1 ? nums[i + 1] : INT_MAX;
//                 s[i % 2] += max(nums[i] - min(left, right) + 1, 0);
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut ans = vec![0,0];
        let len = nums.len();
        for i in 0..len {
            let (mut left, mut right) = (i32::MAX, i32::MAX);
            if i != 0 {
                left = nums[i - 1];
            }
            if i < len-1 {
                right = nums[i + 1];
            }
            ans[i % 2] += 0.max(nums[i] - left.min(right) + 1);
        }
        ans[0].min(ans[1])
    }
}

impl Solution {

    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }
    
        let mut change_even = vec![0; len];
        let mut change_odd = vec![0; len];
        for i in 0..len {
            if i & 1 == 0 {
                if i == 0 {
                    change_odd[i] = nums[1]-1;
                    continue;
                } else if i == len-1 {
                    change_odd[i] = nums[len-2] - 1;
                    continue;
                } else {
                    change_odd[i] = nums[i+1].min(nums[i-1])-1;
                }
                
            } else {
                if i == len-1 {
                    change_even[i] = nums[i-1]-1
                } else {
                    change_even[i] = nums[i+1].min(nums[i-1])-1;
                }
            }
        }
        let mut change_even_sum = 0;
        let mut change_odd_sum = 0;
        for i in 0..len {
            if i & 1 == 0 {
                if nums[i] <= change_odd[i] {
                    continue;
                } else {
                    change_odd_sum += nums[i] - change_odd[i];
                }
            } else {
                if nums[i] <= change_even[i] {
                    continue;
                } else {
                    change_even_sum += nums[i] - change_even[i];
                }
            }
        }
        
        change_even_sum.min(change_odd_sum)
    }
}

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = [0, 0];
        for k in 0..2 {
            for i in (k..n).step_by(2) {
                let mut d = 0;
                if i > 0 {
                    d = d.max(nums[i] - nums[i - 1] + 1);
                }
                if i + 1 < n {
                    d = d.max(nums[i] - nums[i + 1] + 1);
                }
                ans[k] += d
            }
        }
        ans[0].min(ans[1])
    }
}

// class Solution {
//     public:
//         int movesToMakeZigzag(vector<int> &nums) {
//             int s[2]{}, n = nums.size();
//             for (int i = 0; i < n; ++i) {
//                 int left = i ? nums[i - 1] : INT_MAX;
//                 int right = i < n - 1 ? nums[i + 1] : INT_MAX;
//                 s[i % 2] += max(nums[i] - min(left, right) + 1, 0);
//             }
//             return min(s[0], s[1]);
//         }
//     };


// 作者：tuotuoli
// 链接：https://leetcode.cn/problems/decrease-elements-to-make-array-zigzag/solution/1144-di-jian-yuan-su-shi-shu-zu-cheng-ju-chi-zhuan/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

// class Solution:
//     def movesToMakeZigzag(self, nums: List[int]) -> int:
//         def help(pos: int) -> int:
//             res = 0
//             for i in range(pos, len(nums), 2):
//                 a = 0
//                 if i - 1 >= 0:
//                     a = max(a, nums[i] - nums[i - 1] + 1)
//                 if i + 1 < len(nums):
//                     a = max(a, nums[i] - nums[i + 1] + 1)
//                 res += a
//             return res

//         return min(help(0), help(1))