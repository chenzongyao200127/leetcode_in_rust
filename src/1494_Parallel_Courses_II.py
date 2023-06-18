# 1494_Parallel_Courses_II
# https://leetcode.cn/problems/parallel-courses-ii/

# You are given an integer n, 
# which indicates that there are n courses labeled from 1 to n. 
# You are also given an array relations where relations[i] = [prevCoursei, nextCoursei], 
# representing a prerequisite relationship between course prevCoursei and course nextCoursei:
# course prevCoursei has to be taken before course nextCoursei. 
# Also, you are given the integer k.

# In one semester, you can take at most k courses as long as you have taken all the prerequisites 
# in the previous semesters for the courses you are taking.

# Return the minimum number of semesters needed to take all courses. 
# The testcases will be generated such that it is possible to take every course.


# Use backtracking with states (bitmask, degrees) where bitmask represents the set of courses, 
# if the ith bit is 1 then the ith course was taken, 
# otherwise, you can take the ith course. 
# Degrees represent the degree for each course (nodes in the graph).

# Note that you can only take nodes (courses) with degree = 0 
# and it is optimal at every step in the backtracking take the maximum number of courses limited by k.

class Solution:
    def minNumberOfSemesters(self, n: int, relations: List[List[int]], k: int) -> int: