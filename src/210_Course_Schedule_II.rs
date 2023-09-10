// 210_Course_Schedule_II
// https://leetcode.cn/problems/course-schedule-ii/description/


// 现在你总共有 numCourses 门课需要选，记为 0 到 numCourses - 1。给你一个数组 prerequisites 
// 其中 prerequisites[i] = [ai, bi] ，表示在选修课程 ai 前 必须 先选修 bi 。

// 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示：[0,1] 。
// 返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，
// 你只要返回 任意一种 就可以了。如果不可能完成所有课程，返回 一个空数组 。


// 示例 1：
// 输入：numCourses = 2, prerequisites = [[1,0]]
// 输出：[0,1]
// 解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。

// 示例 2：
// 输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
// 输出：[0,2,1,3]
// 解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
// 因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。

// 示例 3：
// 输入：numCourses = 1, prerequisites = []
// 输出：[0]

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prereq_cnts = vec![0; num_courses as usize];
    
        let mut course_map = HashMap::new();
    
        for pair in &prerequisites {
            let (course, prereq_course) = (pair[0] as usize, pair[1] as usize);
            prereq_cnts[course] += 1;
            course_map.entry(prereq_course).or_insert(vec![]).push(course);
        }
    
        let mut queue = VecDeque::new();
    
        for (i, &cnt) in prereq_cnts.iter().enumerate() {
            if cnt == 0 {
                queue.push_back(i);
            }
        }
    
        let mut res = vec![];
    
        while let Some(course) = queue.pop_front() {
            res.push(course as i32);
    
            if let Some(next_course_seqs) = course_map.get(&course) {
                for &next_course in next_course_seqs {
                    prereq_cnts[next_course] -= 1;
                    if prereq_cnts[next_course] == 0 {
                        queue.push_back(next_course);
                    }
                }
            }
        }
    
        if res.len() == num_courses as usize {
            res
        } else {
            vec![]
        }
    }
}