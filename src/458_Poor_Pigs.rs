// 458. Poor Pigs
// https://leetcode.cn/problems/poor-pigs/

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        // return ((buckets as f32).log(rounds as f32 + 1f32)).ceil() as i32;
        return ((buckets as f32).log(rounds as f32 + 1.0).ceil()) as i32
    }
}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        return ((buckets as f32).log(rounds as f32 + 1f32)).ceil() as i32;
    }
}
// 作者：yu-shou-qiu
// 链接：https://leetcode.cn/problems/poor-pigs/solution/458-ke-lian-de-xiao-zhu-zhi-guan-jie-shi-lybw/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。