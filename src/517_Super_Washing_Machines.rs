// 517. Super Washing Machines
// https://leetcode.cn/problems/super-washing-machines/


impl Solution {
    pub fn find_min_moves(mut machines: Vec<i32>) -> i32 {
        let len = machines.len();
        let sum = machines.iter().sum::<i32>();
        if sum % len as i32 != 0 {
            return -1;
        }

        let avg = sum / len as i32;
        let mut ans = 0;
        let mut pre_sum = 0;
        for i in 0..len {
            machines[i] -= avg;
        }
        for num in machines {
            pre_sum += num;
            ans = ans.max(num.max(pre_sum.abs()))
        }
        ans
    }
}


// 方法一：贪心
// class Solution:
//     def findMinMoves(self, machines: List[int]) -> int:
//         tot = sum(machines)
//         n = len(machines)
//         if tot % n:
//             return -1
//         avg = tot // n
//         ans, s = 0, 0
//         for num in machines:
//             num -= avg
//             s += num
//             ans = max(ans, abs(s), num)
//         return ans
// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/super-washing-machines/solution/chao-ji-xi-yi-ji-by-leetcode-solution-yhej/
// 来源：力扣（LeetCode）

// class Solution:
//     def findMinMoves(self, machines: List[int]) -> int:
//         tot, n = sum(machines), len(machines)
//         if tot % n != 0:
//             return -1

//         avg = tot // n
//         ans, presum = 0, 0
//         for i in range(n):
//             presum += machines[i]
//             ans = max(ans, max(abs(presum-(i+1)*avg), machines[i]-avg))
        
//         return ans




// class Solution {
//     public int findMinMoves(int[] machines) {
//         int tot = Arrays.stream(machines).sum();
//         int n = machines.length;
//         if (tot % n != 0) {
//             return -1;
//         }
//         int avg = tot / n;
//         int ans = 0, sum = 0;
//         for (int num : machines) {
//             num -= avg;
//             sum += num;
//             ans = Math.max(ans, Math.max(Math.abs(sum), num));
//         }
//         return ans;
//     }
// }
// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/super-washing-machines/solution/chao-ji-xi-yi-ji-by-leetcode-solution-yhej/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。