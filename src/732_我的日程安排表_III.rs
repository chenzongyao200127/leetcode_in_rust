// 732_我的日程安排表_III
// https://leetcode.cn/problems/my-calendar-iii/description/?envType=daily-question&envId=2025-01-04

// 当 k 个日程存在一些非空交集时（即, k 个日程包含了一些相同时间），就会产生 k 次预订。

// 给你一些日程安排 [startTime, endTime) ，请你在每个日程安排添加后，返回一个整数 k ，表示所有先前日程安排会产生的最大 k 次预订。

// 实现一个 MyCalendarThree 类来存放你的日程安排，你可以一直添加新的日程安排。

// MyCalendarThree() 初始化对象。
// int book(int startTime, int endTime) 返回一个整数 k ，表示日历中存在的 k 次预订的最大值。

struct MyCalendarThree {
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            map: std::collections::BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        *self.map.entry(start_time).or_insert(0) += 1;
        *self.map.entry(end_time).or_insert(0) -= 1;
        let mut ans = 0;
        let mut cnt = 0;
        for &v in self.map.values() {
            cnt += v;
            ans = ans.max(cnt);
        }
        ans
    }
}