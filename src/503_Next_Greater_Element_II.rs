// 503_Next_Greater_Element_II
// https://leetcode.cn/problems/next-greater-element-ii/

// 503. 下一个更大元素 II
// 给定一个循环数组 nums （ nums[nums.length - 1] 的下一个元素是 nums[0] ），返回 nums 中每个元素的 下一个更大元素 。

// 数字 x 的 下一个更大的元素 是按数组遍历顺序，这个数字之后的第一个比它更大的数，
// 这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 -1 。
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut new_nums = nums.clone();
        for i in 0..nums.len()-1 {
            new_nums.push(nums[i]);
        }
        let mut stk = vec![];
        for i in (0..new_nums.len()).rev() {
            while !stk.is_empty() && new_nums[i] >= stk[stk.len()-1] {
                stk.pop();
            }
            if i <= nums.len() - 1 {
                if stk.is_empty() {
                    ans.push(-1);
                } else {
                    ans.push(stk[stk.len()-1]);
                }
            }
            stk.push(new_nums[i]);
        }

        ans.reverse();
        ans
    }
}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let mut ans = vec![-1; nums.len()];
        let mut stack = vec![0usize];
        for i in 1..nums.len() * 2 {
            let ii = i % nums.len();
            while let Some(&n) = stack.last() {
                if nums[n] >= nums[ii] {
                    break;
                }
                ans[n] = nums[ii];
                stack.pop();
            }
            stack.push(ii);
        }
        ans
    }
}
// 作者：tab-liu
// 链接：https://leetcode.cn/problems/next-greater-element-ii/solution/503-xia-yi-ge-geng-da-yuan-su-ii-by-tab-gs95f/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![];
        for i in 0..nums.len() {
            let mut tmp_vec = vec![];
            let mut idx = i + len -1;
            while idx > i {
                tmp_vec.push(nums[idx % len]);
                idx -= 1;   
            }
            while !tmp_vec.is_empty() && tmp_vec[tmp_vec.len()-1] <= nums[i] {
                tmp_vec.pop();
            }
            if tmp_vec.is_empty() {
                ans.push(-1);
            } else {
                ans.push(tmp_vec[tmp_vec.len()-1]);
            }
        }

        ans
    }
}

/// 而在本题中，我们不需要显性地将该循环数组「拉直」，而只需要在处理时对下标取模即可。
// class Solution {
//     public:
//         vector<int> nextGreaterElements(vector<int>& nums) {
//             int n = nums.size();
//             vector<int> ret(n, -1);
//             stack<int> stk;
//             for (int i = 0; i < n * 2 - 1; i++) {
//                 while (!stk.empty() && nums[stk.top()] < nums[i % n]) {
//                     ret[stk.top()] = nums[i % n];
//                     stk.pop();
//                 }
//                 stk.push(i % n);
//             }
//             return ret;
//         }
//     };
    
//     作者：LeetCode-Solution
//     链接：https://leetcode.cn/problems/next-greater-element-ii/solution/xia-yi-ge-geng-da-yuan-su-ii-by-leetcode-bwam/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。