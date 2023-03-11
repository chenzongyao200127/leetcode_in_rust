// 611_Valid_Triangle_Number
// https://leetcode.cn/problems/valid-triangle-number/

// 给定一个包含非负整数的数组 nums ，返回其中可以组成三角形三条边的三元组个数。
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut ans = 0;
        let mut new_nums = nums;
        new_nums.sort_unstable();
        for i in 0..new_nums.len()-2 {
            if new_nums[i] == 0 {
                continue;
            }
            let mut k = i+1;
            for j in i+1..new_nums.len()-1 {
                while k < new_nums.len() && new_nums[i] + new_nums[j] > new_nums[k] {
                    k += 1;
                }
                ans += (k-j-1) as i32
            }
        }
        ans as i32
    }
}



/// 超时
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut new_nums = nums;
        new_nums.sort_unstable();
        for i in 0..new_nums.len()-2 {
            for j in i+1..new_nums.len()-1 {
                for k in j+1..new_nums.len() {
                    if new_nums[k] >= (new_nums[i]+new_nums[j]) || new_nums[i] <= (new_nums[k] - new_nums[j]) {
                        break;
                    } else {
                        // println!("{:?}", (new_nums[i],new_nums[j],new_nums[k]));
                        ans += 1;
                    }
                }
            }
        }
        ans as i32
    }
}


// class Solution {
//     public int triangleNumber(int[] nums) {
//         Arrays.sort(nums);
//         int n = nums.length;
//         int res = 0;
//         for (int i = n - 1; i >= 2; --i) {
//             int l = 0, r = i - 1;
//             while (l < r) {
//                 if (nums[l] + nums[r] > nums[i]) {
//                     res += r - l;
//                     --r;
//                 } else {
//                     ++l;
//                 }
//             }
//         }
//         return res;
//     }
// }

// 作者：jerring
// 链接：https://leetcode.cn/problems/valid-triangle-number/solution/ming-que-tiao-jian-jin-xing-qiu-jie-by-jerring/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。