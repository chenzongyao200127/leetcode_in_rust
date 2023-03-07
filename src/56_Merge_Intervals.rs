// 56. Merge Intervals
// https://leetcode.cn/problems/merge-intervals/


impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        // println!("{:?}", intervals);
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 1..intervals.len() {
            if intervals[i][0] <= right {
                right = right.max(intervals[i][1]);
            } else {
                ans.push(vec![left, right]);
                left = intervals[i][0];
                right = intervals[i][1];
            }
        }
        ans.push(vec![left, right]);

        ans
    }
}


impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut ans = vec![];
        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        intervals.iter().skip(1).for_each(|x| {
            if x[0] > end {
                ans.push(vec![start, end]);
                start = x[0];
            }
            end = end.max(x[1]);
        });
        ans.push(vec![start, end]);
        ans
    }
}
// 作者：tab-liu
// 链接：https://leetcode.cn/problems/merge-intervals/solution/56-he-bing-qu-jian-by-tab-liu-xdg9/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


// class Solution {
//     public int[][] merge(int[][] intervals) {
//         BitSet bitSet = new BitSet();
//         int max = 0;
//         for (int[] interval : intervals) {
//             int temp = interval[1] * 2 + 1;
//             bitSet.set(interval[0] * 2, temp, true);
//             max = temp >= max ? temp : max;
//         }

//         int index = 0, count = 0;
//         while (index < max) {
//             int start = bitSet.nextSetBit(index);
//             int end = bitSet.nextClearBit(start);

//             int[] item = {start / 2, (end - 1) / 2};
//             intervals[count++] = item;

//             index = end;
//         }
//         int[][] ret = new int[count][2];
//         for (int i = 0; i < count; i++) {
//             ret[i] = intervals[i];
//         }

//         return ret;
//     }
// }


impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let len = intervals.len();
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 1..intervals.len() {
            if intervals[i][0] <= right {
                right = intervals[i][1];
            } else {
                ans.push(vec![left, right]);
                left = intervals[i][0];
                left = intervals[i][1];
            }
        }

        ans
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