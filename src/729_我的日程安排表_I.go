// 3280. 将日期转换为二进制表示

// 给你一个字符串 date，它的格式为 yyyy-mm-dd，表示一个公历日期。
// date 可以重写为二进制表示，只需要将年、月、日分别转换为对应的二进制表示（不带前导零）并遵循 year-month-day 的格式。
// 返回 date 的 二进制 表示。

func convertDateToBinary(date string) string {
	year, _ := strconv.Atoi(date[:4])
	month, _ := strconv.Atoi(date[5:7])
	day, _ := strconv.Atoi(date[8:10])
	return strconv.FormatInt(int64(year), 2) + "-" + strconv.FormatInt(int64(month), 2) + "-" + strconv.FormatInt(int64(day), 2)
}


// 实现一个 MyCalendar 类来存放你的日程安排。如果要添加的日程安排不会造成 重复预订 ，则可以存储这个新的日程安排。

// 当两个日程安排有一些时间上的交叉时（例如两个日程安排都在同一时间内），就会产生 重复预订 。

// 日程可以用一对整数 startTime 和 endTime 表示，这里的时间是半开区间，即 [startTime, endTime), 实数 x 的范围为，  startTime <= x < endTime 。

// 实现 MyCalendar 类：

// MyCalendar() 初始化日历对象。
// boolean book(int startTime, int endTime) 如果可以将日程安排成功添加到日历中而不会导致重复预订，返回 true 。否则，返回 false 并且不要将该日程安排添加到日历中。

// 0 <= start < end <= 10^9
// 每个测试用例，调用 book 方法的次数最多不超过 1000 次。

type MyCalendar struct {
	memo map[int]int
}

func Constructor() MyCalendar {
	return MyCalendar{
		memo: make(map[int]int),
	}
}

func (this *MyCalendar) Book(startTime int, endTime int) bool {
	for k, v := range this.memo {
		if startTime < v && endTime > k {
			return false
		}
	}
	this.memo[startTime] = endTime
	return true
}


/**
 * Your MyCalendar object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Book(startTime,endTime);
 */