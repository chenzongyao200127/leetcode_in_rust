// 860_Lemonade_Change
// https://leetcode.cn/problems/lemonade-change/

// 在柠檬水摊上，每一杯柠檬水的售价为 5 美元。顾客排队购买你的产品，（按账单 bills 支付的顺序）一次购买一杯。

// 每位顾客只买一杯柠檬水，然后向你付 5 美元、10 美元或 20 美元。你必须给每个顾客正确找零，也就是说净交易是每位顾客向你支付 5 美元。

// 注意，一开始你手头没有任何零钱。

// 给你一个整数数组 bills ，其中 bills[i] 是第 i 位顾客付的账。如果你能给每位顾客正确找零，返回 true ，否则返回 false 。


impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = vec![0;2];
        for bill in bills {
            match bill {
                5 => change[0] += 1,
                10 => {
                    if change[0] > 0 {
                        change[0] -= 1;
                        change[1] += 1;
                    } else {
                        return false;
                    }
                }
                _ => {
                    if change[1] > 0 {
                        change[1] -= 1;
                        if change[0] > 0 {
                            change[0] -= 1;
                        } else {
                            return false
                        }
                    } else if change[0] >= 3 {
                        change[0] -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}


// 可读性优化
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = [0, 0];
        for bill in bills {
            match bill {
                5 => change[0] += 1,
                10 if change[0] > 0 => {
                    change[0] -= 1;
                    change[1] += 1;
                }
                10 => return false,
                20 if change[1] > 0 && change[0] > 0 => {
                    change[1] -= 1;
                    change[0] -= 1;
                }
                20 if change[0] >= 3 => {
                    change[0] -= 3;
                }
                _ => return false,
            }
        }
        true
    }
}