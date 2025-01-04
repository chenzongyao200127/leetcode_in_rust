// 731_我的日程安排表_II

// 实现一个程序来存放你的日程安排。如果要添加的时间内不会导致三重预订时，则可以存储这个新的日程安排。

// 当三个日程安排有一些时间上的交叉时（例如三个日程安排都在同一时间内），就会产生 三重预订。

// 事件能够用一对整数 startTime 和 endTime 表示，在一个半开区间的时间 [startTime, endTime) 上预定。实数 x 的范围为  startTime <= x < endTime。

// 实现 MyCalendarTwo 类：

// MyCalendarTwo() 初始化日历对象。
// boolean book(int startTime, int endTime) 如果可以将日程安排成功添加到日历中而不会导致三重预订，返回 true。否则，返回 false 并且不要将该日程安排添加到日历中。

type pair struct{ start, end int }
type MyCalendarTwo struct{ booked, overlaps []pair }

func Constructor() MyCalendarTwo {
    return MyCalendarTwo{}
}

func (c *MyCalendarTwo) Book(start, end int) bool {
    for _, p := range c.overlaps {
        if p.start < end && start < p.end {
            return false
        }
    }
    for _, p := range c.booked {
        if p.start < end && start < p.end {
            c.overlaps = append(c.overlaps, pair{max(p.start, start), min(p.end, end)})
        }
    }
    c.booked = append(c.booked, pair{start, end})
    return true
}

func min(a, b int) int {
    if a > b {
        return b
    }
    return a
}

func max(a, b int) int {
    if b > a {
        return b
    }
    return a
}
