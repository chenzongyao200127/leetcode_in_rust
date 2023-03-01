// 476. Number Complement
// https://leetcode.cn/problems/number-complement/

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        ((1 << (32 - num.leading_zeros())) as i32 - 1) ^ num
    }
}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        ((1 << (32 - num.leading_zeros())) as i32 - 1) - num
    }
}

impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut t = num;
        t |= t >> 1;
        t |= t >> 2;
        t |= t >> 4;
        t |= t >> 8;
        t |= t >> 16;
        return !num & t;
    }
}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut all_one = i32::MAX;
        while all_one >> 1 >= num {
            all_one >>= 1;
        }
        all_one - num
    }
}
// 作者：phpgoc
// 链接：https://leetcode.cn/problems/number-complement/solution/quan-1you-yi-by-phpgoc-nxkt/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。