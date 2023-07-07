# 2532. 过桥的时间
# https://leetcode.cn/problems/time-to-cross-a-bridge/description/


# 方法一：优先队列
# 思路与算法

# 在本题中，工人共有 4 种状态：

# 在左侧等待
# 在右侧等待
# 在左侧工作（放下所选箱子）
# 在右侧工作（搬起所选箱子）

# 每一种工作状态都有相应的优先级计算方法，因此我们用 4 个优先队列来存放处于每种状态下的工人集合：

# 在左侧等待的工人：wait_left，题目中定义的效率越高，优先级越高。
# 在右侧等待的工人：wait_right，题目中定义的效率越高，优先级越高。
# 在左侧工作的工人：work_left，完成时间越早，优先级越高。
# 在右侧工作的工人：work_right，完成时间越早，优先级越高。

# 我们令 remain 表示右侧还有多少个箱子需要搬运，当 remain>0 时，搬运工作需要继续。
# 除此之外，题目求解的是最后一个回到左边的工人的到达时间，
# 因此当右侧还有工人在等待或在工作时（即 work_right或 wait_right 不为空），搬运工作就需要继续：

# 若 work_left中的工人在此刻已经完成工作，则需要将它们取出并分别加入到 wait_left 和 wait_right 中。
# 若 wait_right 不为空，则取其中优先级最低的工人过桥，将其加入到 work_left 队列中，并将时间更改为过桥后的时间。继续下一轮循环。
# 若 remain>0，并且 wait_left 不为空，则需要取优先级最低的工人过桥去取箱子，将其加入到 work_right 队列中，
# 令 remain 减 1，并将时间更改为过桥后的时间。继续下一轮循环。
# 若 2 和 3 都不满足，表示当前没有人需要过桥，当前时间应该过渡到 work_left 和 work_right 中的最早完成时间。然后继续下一轮循环。


# 这是一个复杂模拟题，值得标Hard。“等待中”和“工作中”各需要2个优先队列（注意关键字有所不同），
# 每次循环开始时，先根据当前时间节点判断两侧是否有“工作中”的工人变为”等待中“，
# 然后再从”等待中“的工人里按规则选出过桥的那个，再将其加入桥另一侧的”工作中“。
# 如果没有”等待中“，那么当前时间就要快进到最近的会出现”等待中“的时间点。

from typing import List

import heapq

class Solution:
    def findCrossingTime(self, n: int, k: int, time: List[List[int]]) -> int:
        # 将每个工人的编号附加到 time 列表中，以便追踪
        for i in range(len(time)):
            time[i].append(i)
            
        # 根据工人往返桥的时间对 time 进行排序
        time.sort(key=lambda x: [-x[0]-x[2], -x[4]])

        current_time = 0
        left_side_waiting = []  # 在桥左侧等待的工人
        right_side_waiting = []  # 在桥右侧等待的工人
        left_side_heap = [[j, 0] for j in range(k)]  # 桥左侧的工人堆
        right_side_heap = []  # 桥右侧的工人堆
        heapq.heapify(left_side_heap)  # 将列表转换为堆
        boxes_left = n
        boxes_moved = 0

        while boxes_left:
            if right_side_heap:  # 如果右侧堆中有工人
                worker_idx, _ = heapq.heappop(right_side_heap)
                heapq.heappush(left_side_waiting, [current_time + time[worker_idx][2] + time[worker_idx][3], worker_idx])
                current_time += time[worker_idx][2]
                boxes_left -= 1
            elif left_side_heap and boxes_moved < n:  # 如果左侧堆中有工人并且尚未移动所有箱子
                worker_idx, _ = heapq.heappop(left_side_heap)
                heapq.heappush(right_side_waiting, [current_time + time[worker_idx][0] + time[worker_idx][1], worker_idx])
                current_time += time[worker_idx][0]
                boxes_moved += 1
            else:
                # 将当前时间前进到下一个重要时间点 - 当前左侧任务的结束或右侧任务的结束
                current_time = min(
                    left_side_waiting[0][0] if left_side_waiting and boxes_moved < n else 10**9, 
                    right_side_waiting[0][0] if right_side_waiting else 10**9
                )

            # 处理所有可以在当前时间完成的右侧工人
            while right_side_waiting and right_side_waiting[0][0] <= current_time:
                heapq.heappush(right_side_heap, [right_side_waiting[0][1], right_side_waiting[0][0]])
                heapq.heappop(right_side_waiting)
                
            # 处理所有可以在当前时间完成的左侧工人
            while left_side_waiting and left_side_waiting[0][0] <= current_time:
                heapq.heappush(left_side_heap, [left_side_waiting[0][1], left_side_waiting[0][0]])
                heapq.heappop(left_side_waiting)

        return current_time
