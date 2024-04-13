# 阿里 0413 笔试灵犀互娱-25届春招-游戏开发&引擎 2.5小时
# 20道选择，感觉比之前的简单一些
# 5道编程：
# 1. 给两个数组，计算两个（a, b）的最多数量（a 属于 第一个数组，b 属于第二个数组，a <= b， a、b只能取一次）（签到题，双指针）
# 2. 给定多个查询（left, right），每次查询会使得 查询范围内的值 + 1，计算所有查询结束后的和 （签到题）
# 3. 判断s1的排列是否是s2的字串（hastmap + 滑动窗口）
# 4. 给定一个数组 num，和很多查询（left, right），判断 num[left-1..right] 中的值能否构成三角形 （过了65%，不知道查询怎么优化速度）
# 5. 给定一个带权无向图，找出给定source、target 俩节点之间所有路径中，{路途最大权值 / 路途最小权值} 最小的值，返回最简分数 （dfs，超时，只过了10%）

from fractions import Fraction
from collections import defaultdict, deque
import heapq
from os import curdir
from functools import cache
import sys

line = list(map(int, sys.stdin.readline().strip().split(" ")))
people_num = line[0]
querys_num = line[1]

# print(people_num, querys_num)

peoples = [0] * people_num
for i in range(people_num):
    T = int(sys.stdin.readline().strip().split(" ")[0])
    peoples[i] = T

# print(peoples)


@cache
def helper(nums):
    if len(nums) < 3:
        return False

    nums = list(nums)
    nums = [x for x in nums if x > 0]
    nums.sort()

    for i in range(len(nums) - 2):
        if nums[i] + nums[i + 1] > nums[i + 2]:
            return True

    return False


res = ""
for i in range(querys_num):
    line = list(map(int, sys.stdin.readline().strip().split(" ")))
    l = line[0]
    r = line[1]
    # print(peoples[l-1:r])
    if helper(tuple(peoples[l - 1: r])):
        res += "Y"
    else:
        res += "N"

print(res)


T = int(sys.stdin.readline().strip().split(" ")[0])


def add_edge(g, u, v, w):
    g[u].append((v, w))
    g[v].append((u, w))


def helper(g, s, e):
    paths = []
    path_ratios = []

    def dfs(current, e, path, visited, max_weight, min_weight):
        path.append(current)
        visited.add(current)
        # print(path, visited)

        if current == e:
            if max_weight > 0 and min_weight < float('inf'):
                ratio = Fraction(max_weight, min_weight)
                paths.append(path[:])
                path_ratios.append(ratio)
            # path.pop()
            # return

        for nei, weight in g[current]:
            if nei not in visited:
                dfs(nei, e, path, visited, max(
                    max_weight, weight), min(min_weight, weight))

        path.pop()
        visited.remove(current)

    visited = set()
    dfs(s, e, [], visited, 0, float('inf'))

    return paths, path_ratios


for _ in range(T):
    line = list(map(int, sys.stdin.readline().strip().split(" ")))
    n = line[0]
    m = line[1]
    line = list(map(int, sys.stdin.readline().strip().split(" ")))
    source = line[0]
    end = line[1]
    g = dict()
    for k in range(1, n+1):
        g[k] = []
    for _ in range(m):
        line = list(map(int, sys.stdin.readline().strip().split(" ")))
        fr = line[0]
        to = line[1]
        value = line[2]
        add_edge(g, fr, to, value)

    # print(g)
    paths, path_ratios = helper(g, source, end)
    # print(paths, path_ratios)
    if not paths:
        print("%%%")
    else:
        print(min(path_ratios))
