// 75. Sort Colors
// https://leetcode.cn/problems/sort-colors/

// 75. 颜色分类
// 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
// 我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
// 必须在不使用库内置的 sort 函数的情况下解决这个问题。

// 说下我的理解：三个指针num1、num2、num3将数组nums分成了3个分区，从左往右依次存储0、1、2。 
// 三个指针分别指向各自分区的尾部。 从左到右遍历数组nums，
/// (1)如果nums[i] = 0,则1、2区都后移一个位置，给新来的0腾地方。
/// (2)如果是nums[i] = 1，同样，2区后移一个位置，给新来的1腾地方。前面的0区无影响。
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut num0, mut num1, mut num2) = (0, 0, 0);
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums[num2] = 2;
                nums[num1] = 1;
                nums[num0] = 0;
                num0 += 1;
                num1 += 1;
                num2 += 1;
            }else if nums[i] == 1  {
                nums[num2] = 2;
                nums[num1] = 1;
                num1 += 1;
                num2 += 1;
            }else {
                nums[num2] = 2;
                num2 += 1;
            }
        }
    }
}



// 双指针，算法导论给出的解法，对应官方解法2
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1{ return; }
        let mut eq_l = 0;
        let mut eq_r = 0;
        for i in 0..nums.len(){
            if nums[i] < 1 {
                nums.swap(i, eq_l);
                if eq_l < eq_r {
                    nums.swap(eq_r, i);
                }
                eq_l += 1;
                eq_r += 1;
            } else if nums[i] == 1 {
                nums.swap(i, eq_r);
                eq_r += 1;
            }
        }
    }
}


// 三指针指向边界
// class Solution {
//     public void sortColors(int[] nums) {
//         int num0 = 0, num1 = 0, num2 = 0;
//         for(int i = 0; i < nums.length; i++) {
//             if(nums[i] == 0) {
//                 nums[num2++] = 2;
//                 nums[num1++] = 1;
//                 nums[num0++] = 0;
//             }else if(nums[i] == 1) {
//                 nums[num2++] = 2;
//                 nums[num1++] = 1;
//             }else {
//                 nums[num2++] = 2;
//             }
//         }
//     }
// }



// 0，1，2 排序。一次遍历，如果是0，则移动到表头，如果是2，则移动到表尾，不用考虑1。0和2处理完，1还会有错吗？

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut c0, mut c1) = (0, 0);
        for &n in nums.iter() {
            if n == 0 {
                c0 += 1;
            } else if n == 1 {
                c1 += 1;
            }
        }
        for i in 0..c0 {
            nums[i] = 0;
        }
        for i in c0..c0 + c1 {
            nums[i] = 1;
        }
        for i in c0 + c1..nums.len() {
            nums[i] = 2;
        }
    }
}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1{ return; }
        let mut eq_l = 0;
        let mut eq_r = 0;
        for i in 0..nums.len() {
            if nums[i] < 1 {
                nums.swap(i, eq_l);
                if eq_l < eq_r {
                    nums.swap(eq_r, i);
                }
                eq_l += 1;
                eq_r += 1;
            } else if nums[i] == 1{
                nums.swap(i, eq_r);
                eq_r += 1;
            }
        }
        
    }
}
// 作者：mochamapleaf
// 链接：https://leetcode.cn/problems/sort-colors/solution/by-mochamapleaf-8t10/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。