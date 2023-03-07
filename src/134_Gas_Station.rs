// 134_Gas_Station
// https://leetcode.cn/problems/gas-station/

// 134. 加油站
// 在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
// 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。
// 给定两个整数数组 gas 和 cost ，如果你可以绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut diff = vec![];
        for i in 0..gas.len() {
            diff.push(gas[i] - cost[i]);
        }
        diff = diff.iter().enumerate().collect()
        diff.sort_unstable(|x| x.);
    }
}