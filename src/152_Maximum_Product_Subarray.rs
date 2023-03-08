// 152_Maximum_Product_Subarray
// https://leetcode.cn/problems/maximum-product-subarray/


// 152. 乘积最大子数组
// 给你一个整数数组 nums ，请你找出数组中乘积最大的非空连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。
// 测试用例的答案是一个 32-位 整数。
// 子数组 是数组的连续子序列。

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let (mut max, mut min) = (1, 1);
        for n in nums {
            if n < 0 {
                std::mem::swap(&mut max, &mut min);
            }
            max = n.max(max * n);
            min = n.min(min * n);
            ans = ans.max(max);
        }
        ans
    }
}


// 解题思路：
// 这题是求数组中子区间的最大乘积，对于乘法，我们需要注意，负数乘以负数，会变成正数，所以解这题的时候我们需要维护两个变量，
// 当前的最大值，以及最小值，最小值可能为负数，但没准下一步乘以一个负数，当前的最大值就变成最小值，而最小值则变成最大值了。

// 我们的动态方程可能这样：
// maxDP[i + 1] = max(maxDP[i] * A[i + 1], A[i + 1], minDP[i] * A[i + 1])
// minDP[i + 1] = min(minDP[i] * A[i + 1], A[i + 1], maxDP[i] * A[i + 1])
// dp[i + 1] = max(dp[i], maxDP[i + 1])

// 这里，我们还需要注意元素为 0 的情况，如果 A[i] 为 0，那么 maxDP 和 minDP 都为 0，
// 我们需要从 A[i + 1] 重新开始。
// class Solution {
//     public:
//         int maxProduct(vector<int>& nums) {
//             int n = nums.size();
//             if(n == 0){
//                 return 0;
//             } else if(n == 1) {
//                 return nums[0];
//             }
//             int p = nums[0];
//             int maxP = nums[0];
//             int minP = nums[0];
//             for(int i = 1; i < n; i++) {
//                 int t = maxP;
//                 maxP = max(max(maxP * nums[i], nums[i]), minP *nums[i]);
//                 minP = min(min(t * nums[i], nums[i]), minP * nums[i]);
//                 p = max(maxP, p);
//             }
//             return p;
//         }
//     };
    
//     作者：spicy_onion
//     链接：https://leetcode.cn/problems/maximum-product-subarray/solution/dpfang-fa-xiang-jie-by-yang-cong-12/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut imax, mut imin) = (-i32::MAX, 1, 1);

        for i in nums {
            if i < 0 {
                // 交换值
                let mut temp = imax;
                imax = imin;
                imin = temp;
            }

            imax = i.max(i*imax);
            imin = i.min(i*imin);

            max = max.max(imax);
        }

        max
    }
}


/// Failed! 暴力枚举，把0作为分割点，记录每个分割组中负数的个数（若为奇数，则定位第一个和最后一个负数的位置，...）（若为偶数，则所有元素累乘）
// impl Solution {
//     pub fn max_product(nums: Vec<i32>) -> i32 {
//         if nums == vec![0] {
//             return 0;
//         }
//         let mut groups = vec![];
//         let mut group = vec![];
//         let mut ans = i32::MIN;
//         for i in 0..nums.len() {
//             if nums[i] != 0 {
//                 group.push(nums[i]);
//             } else {
//                 groups.push(group.clone());
//                 group.clear();
//             }
//         }
//         groups.push(group.clone());
//         if groups.len() != 1 {
//             ans = 0;
//         }
//         // println!("{:?}", groups);
//         for group in groups {
//             let mut neg_cnt = 0;
//             let mut max = 1;
//             let mut max3 = 0;
//             for num in group.iter() {
//                 if *num < 0 {
//                     neg_cnt += 1;
//                 }
//             }
//             if neg_cnt & 1 == 0 {
//                 for num in group.iter() {
//                     max *= num;
//                 }
//                 ans = ans.max(max)
//             } else {
//                 let mut first = 0;
//                 let mut last = group.len()-1;
//                 let mut max1 = 1;
//                 let mut max2 = 1;
//                 for i in 0..group.len() {
//                     if group[i] < 0 {
//                         first = i;
//                         break;
//                     }
//                 }
//                 for j in (0..group.len()).rev() {
//                     if group[j] < 0 {
//                         last = j;
//                         break;
//                     }
//                 }
//                 if first == group.len() {
//                     max1 = i32::MIN;
//                 } else {
//                     for i in first+1..group.len() {
//                         max1 *= group[i];
//                     }
//                 }
//                 if last == 0 {
//                     max2 = i32::MIN;
//                 } else {
//                     for j in 0..last {
//                         max2 *= group[j];
//                     }
//                 }
//                 if first == last && last == 0 {
//                     max1 = group[0];
//                     max2 = group[0];
//                 }
//                 // println!("{:?}", (max1, max2));
//                 max3 = max1.max(max2);
//             }
//             // println!("{:?}", ans);
//             // println!("{:?}", max);
//             ans = ans.max(max3);
//         }
    
//         ans
//     }
// }