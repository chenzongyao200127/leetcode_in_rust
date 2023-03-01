// 507. Perfect Number
// https://leetcode.cn/problems/perfect-number/

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
    
        let mut sum = 0;
        for i in 2..((num as f64).sqrt() as i32 + 1) {
            if num % i == 0 {
                if i*i == num {
                    sum += i;
                } else {
                    sum += i + num/i
                }
            }
        }
    
        sum + 1 == num
    }
}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        match num {
            6 | 28 | 496 | 8128 | 33550336 => true,
            _ => false,
        }
    }
}
// 作者：kyushu
// 链接：https://leetcode.cn/problems/perfect-number/solution/rustgolangjava-da-biao-bao-li-mei-ju-you-gdr3/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。