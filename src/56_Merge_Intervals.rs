// 56. Merge Intervals
// https://leetcode.cn/problems/merge-intervals/


impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    }
}






// Failed 输入：
// [[1,4],[5,6]]
// 输出：
// [[1,6]]
// 预期结果：
// [[1,4],[5,6]]

// impl Solution {
//     pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let mut bitmap = vec![0;10001];
//         let mut ans = vec![];
//         for interval in intervals {
//             let start = interval[0] as usize;
//             let end = interval[1] as usize;
//             for i in start..=end {
//                 bitmap[i] = 1;
//             }    
//         }
//         let mut tmp = vec![];
//         for i in 0..bitmap.len()-1 {
//             if bitmap[i] != bitmap[i+1] {
//                 if bitmap[i] == 1 {
//                     tmp.push(i as i32);
//                 } else {
//                     tmp.push(i as i32 + 1);
//                 }
//             }
//         }
//         for i in (0..tmp.len()-1).filter(|x| (x % 2 == 0)) {
//             ans.push(vec![tmp[i], tmp[i+1]]);
//         }
    
//         ans
//     }
// }