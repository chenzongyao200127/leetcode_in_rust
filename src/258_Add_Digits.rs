// 258. Add Digits
// https://leetcode.cn/problems/add-digits/

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let mut new_num = 0;
        let mut tmp = num / 10;
        while num != 0 {
            new_num += num % 10;
            num = num / 10;
            tmp = num % 10;
        }

        return Solution::add_digits(new_num)
    }
}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 { num } else { Solution::add_digits(num / 10 + num % 10) }
    }
}
// 作者：kyushu
// 链接：https://leetcode.cn/problems/add-digits/solution/rustgojava-xun-huan-di-gui-zhao-gui-lu-1-ye3j/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        if num >= 9 && num % 9 == 0{
            9
        }
        else {
            num % 9
        }
    }
}


impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        return (num - 1) % 9 + 1;
    }
}