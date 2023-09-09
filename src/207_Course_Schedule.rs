// 207_Course_Schedule
// https://leetcode.cn/problems/course-schedule/description/

// There are a total of numCourses courses you have to take, 
// labeled from 0 to numCourses - 1. 
// You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates 
// that you must take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return true if you can finish all courses. Otherwise, return false.

// Example 1:
// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: true
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0. So it is possible.

// Example 2:
// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
// Output: false
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0, an

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // prerequisites_count 用于存储每门课程所需的先修课程数量。
        let mut prerequisites_count = vec![0; num_courses as usize];
        
        // course_map 映射了一门课程到一列依赖于这门课程的后续课程。
        let mut course_map = HashMap::new();
        
        // 根据给定的先修课程关系，填充 prerequisites_count 和 course_map。
        for pair in &prerequisites {
            let (course, prerequisite) = (pair[0] as usize, pair[1] as usize);
            prerequisites_count[course] += 1;
            course_map.entry(prerequisite).or_insert(vec![]).push(course);
        }

        // 队列用于存放没有先修课程限制的课程。
        let mut queue = VecDeque::new();
        
        // 将没有先修课程的课程添加到队列中。
        for (index, &count) in prerequisites_count.iter().enumerate() {
            if count == 0 {
                queue.push_back(index);
            }
        }

        // 记录可以选的课程数量。
        let mut taken_courses = 0;

        // 如果当前课程没有先修课程限制，就将其从队列中删除，并为取出的课程增加已选课程计数。
        while let Some(course) = queue.pop_front() {
            taken_courses += 1;
            
            // 对于每个依赖于当前课程的后续课程，减少其 prerequisites_count。
            // 如果某后续课程的 prerequisites_count 减少到 0，则将其加入队列。
            if let Some(subsequent_courses) = course_map.get(&course) {
                for &subsequent_course in subsequent_courses {
                    prerequisites_count[subsequent_course] -= 1;
                    if prerequisites_count[subsequent_course] == 0 {
                        queue.push_back(subsequent_course);
                    }
                }
            }
        }

        // 如果我们可以选所有的课程，返回 true，否则返回 false。
        taken_courses == num_courses
    }
}
