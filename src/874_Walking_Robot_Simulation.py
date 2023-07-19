# 874. Walking Robot Simulation
# https://leetcode.cn/problems/walking-robot-simulation/

# 机器人在一个无限大小的 XY 网格平面上行走，从点 (0, 0) 处开始出发，面向北方。该机器人可以接收以下三种类型的命令 commands ：

# -2 ：向左转 90 度
# -1 ：向右转 90 度
# 1 <= x <= 9 ：向前移动 x 个单位长度
# 在网格上有一些格子被视为障碍物 obstacles 。第 i 个障碍物位于网格点  obstacles[i] = (xi, yi) 。

# 机器人无法走到障碍物上，它将会停留在障碍物的前一个网格方块上，但仍然可以继续尝试进行该路线的其余部分。

# 返回从原点到机器人所有经过的路径点（坐标为整数）的最大欧式距离的平方。（即，如果距离为 5 ，则返回 25 ）

# 注意：

# 北表示 +Y 方向。
# 东表示 +X 方向。
# 南表示 -Y 方向。
# 西表示 -X 方向。

# 输入：commands = [4,-1,4,-2,4], obstacles = [[2,4]]
# 输出：65
# 解释：机器人开始位于 (0, 0)：
# 1. 向北移动 4 个单位，到达 (0, 4)
# 2. 右转
# 3. 向东移动 1 个单位，然后被位于 (2, 4) 的障碍物阻挡，机器人停在 (1, 4)
# 4. 左转
# 5. 向北走 4 个单位，到达 (1, 8)
# 距离原点最远的是 (1, 8) ，距离为 12 + 82 = 65
from typing import List

class Solution:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        obstacles_set = set(map(tuple, obstacles))
        cur_loca = [0, 0, 0]
        ans = 0
        for com in commands:
            if com == -1:
                cur_dir = cur_loca[2]
                dir = (cur_dir + 1) % 4
                cur_loca[2] = dir
            elif com == -2:
                cur_dir = cur_loca[2]
                dir = (cur_dir - 1) % 4
                cur_loca[2] = dir
            else:
                cur_dir = cur_loca[2]
                for _ in range(com):
                    if cur_dir == 0:
                        cur_loca[1] += 1
                    elif cur_dir == 1:
                        cur_loca[0] += 1
                    elif cur_dir == 2:
                        cur_loca[1] -= 1
                    else:
                        cur_loca[0] -= 1
                    if tuple(cur_loca[:2]) in obstacles_set:
                        if cur_dir == 0:
                            cur_loca[1] -= 1
                        elif cur_dir == 1:
                            cur_loca[0] -= 1
                        elif cur_dir == 2:
                            cur_loca[1] += 1
                        else:
                            cur_loca[0] += 1
                        ans = max(ans, cur_loca[0] ** 2 + cur_loca[1] ** 2 )
                        print(cur_loca)
                        break
                print(cur_loca)
                ans = max(ans, cur_loca[0] ** 2 + cur_loca[1] ** 2 )
                
        return ans

# 优化：
from typing import List
class Solution:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        obstacles_set = set(map(tuple, obstacles))
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        cur_loc = (0, 0)
        cur_dir = 0
        ans = 0
        
        for com in commands:
            if com == -1:
                cur_dir = (cur_dir + 1) % 4
            elif com == -2:
                cur_dir = (cur_dir - 1) % 4
            else:
                dx, dy = directions[cur_dir]
                for _ in range(com):
                    new_loc = (cur_loc[0] + dx, cur_loc[1] + dy)
                    if new_loc in obstacles_set:
                        break
                    cur_loc = new_loc
                ans = max(ans, cur_loc[0] ** 2 + cur_loc[1] ** 2)
                
        return ans