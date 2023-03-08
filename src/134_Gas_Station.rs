// 134_Gas_Station
// https://leetcode.cn/problems/gas-station/

// 134. 加油站
// 在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
// 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。
// 给定两个整数数组 gas 和 cost ，如果你可以绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。

/// 来自评论区
// 有一个环形路上有n个站点； 每个站点都有一个好人或一个坏人； 好人会给你钱，坏人会收你一定的过路费，如果你带的钱不够付过路费，坏人会跳起来把你砍死； 问：从哪个站点出发，能绕一圈活着回到出发点?
// 首先考虑一种情况：如果全部好人给你 的钱加起来 小于 坏人收的过路费之和，那么总有一次你的钱不够付过路费，你的结局注定会被砍死。
// 假如你随机选一点 start 出发，那么你肯定会选一个有好人的站点开始，因为开始的时候你没有钱，遇到坏人只能被砍死；
// 现在你在start出发，走到了某个站点end，被end站点的坏人砍死了，说明你在 [start, end) 存的钱不够付 end点坏人的过路费，因为start站点是个好人，所以在 (start, end) 里任何一点出发，你存的钱会比现在还少，还是会被end站点的坏人砍死；
// 于是你重新读档，聪明的选择从 end+1点出发，继续你悲壮的征程； 终于有一天，你发现自己走到了尽头（下标是n-1)的站点而没有被砍死； 此时你犹豫了一下，那我继续往前走，身上的钱够不够你继续走到出发点Start?
// 当然可以，因为开始已经判断过，好人给你的钱数是大于等于坏人要的过路费的，你现在攒的钱完全可以应付 [0, start) 这一段坏人向你收的过路费。 这时候你的嘴角微微上扬，眼眶微微湿润，因为你已经知道这个世界的终极奥秘：Start就是这个问题的答案。

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut i = 0;
        while i < len {
            let mut count = 0;
            let mut j = i;
            let mut gasSum = 0;
            let mut costSum = 0;
            while count < len {
                j = (count + i) % len;
                gasSum = gasSum + gas[j];
                costSum = costSum + cost[j];
                if gasSum >= costSum {
                    count = count + 1;
                } else {
                    break;
                }
            }
            if count == len {
                return i as i32;
            } else {
                i = i + count + 1;
            }
        }
        return -1;
    }
}


/// 没看懂这里的写法
// impl Solution {
//     pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//         let mut cur = 0;
//         let (mut sum, mut pre) = (0, 0);
//         let retain: Vec<i32> = gas.iter().zip(cost).map(|(&x, y)| x - y).collect();

//         for (i, &n) in retain.iter().enumerate() {
//             sum += n;
//             if sum < 0 {
//                 pre += sum;
//                 sum = 0;
//                 cur = i + 1;
//             }
//         }
//         if pre + sum < 0 {
//             -1
//         } else {
//             cur as i32
//         }
//     }
// }
// // 作者：tab-liu
// // 链接：https://leetcode.cn/problems/gas-station/solution/134-jia-you-zhan-by-tab-liu-hdke/
// // 来源：力扣（LeetCode）
// // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。