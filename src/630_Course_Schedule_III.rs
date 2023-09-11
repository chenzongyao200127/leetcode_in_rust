// 630_Course_Schedule_III
// https://leetcode.cn/problems/course-schedule-iii/description/
use std::collections::BinaryHeap;


impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        // 过滤并排序课程
        let mut courses: Vec<_> = courses.into_iter()
            .filter(|course| course[0] <= course[1])
            .collect();
        courses.sort_by_key(|c| c[1]);

        if courses.is_empty() {
            return 0;
        }

        let mut cur_time = 0;
        let mut pq = BinaryHeap::new();  // 最大堆

        for course in courses {
            let (duration, ddl) = (course[0], course[1]);
            if cur_time + duration <= ddl {
                cur_time += duration;
                pq.push(duration);
            } else {
                // 注意，由于Rust的BinaryHeap是最大堆，我们直接取最大值
                if let Some(&d) = pq.peek() {
                    if duration < d {
                        cur_time = cur_time - d + duration;
                        pq.pop();
                        pq.push(duration);
                    }
                }
            }
        }

        pq.len() as i32
    }
}
