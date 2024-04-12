# 253_会议室_II
# https://leetcode.cn/problems/meeting-rooms-ii/description/?envType=weekly-question&envId=2024-04-01

from typing import List


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        l = -1
        r = 0
        for inter in intervals:
            s = inter[0]
            e = inter[1]
            l = min(l, s)
            r = max(r, e)

        le = r - l + 1
        cnt = [0] * le
        for inter in intervals:
            s = inter[0]
            e = inter[1]
            for i in range(s-le, e-le):
                cnt[i] += 1

        return max(cnt)


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        if not intervals:
            return 0

        # Create a list to store changes in room count
        changes = []
        for s, e in intervals:
            changes.append((s, 1))  # Meeting starts, need one more room
            changes.append((e, -1))  # Meeting ends, need one less room

        # Sort the changes first by time, then by type of change ('end' should come before 'start' if times are the same)
        changes.sort(key=lambda x: (x[0], x[1]))

        # Apply the changes to compute the maximum number of rooms needed
        max_rooms = 0
        current_rooms = 0
        for time, change in changes:
            current_rooms += change
            max_rooms = max(max_rooms, current_rooms)

        return max_rooms


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:

        # 如果没有会议，我们不需要任何房间。
        if not intervals:
            return 0

        used_rooms = 0

        # 将开始计时和结束计时分开，并分别对它们进行排序。
        start_timings = sorted([i[0] for i in intervals])
        end_timings = sorted(i[1] for i in intervals)
        L = len(intervals)

        # 算法中的两个指针：e_ptr 和 s_ptr。
        end_pointer = 0
        start_pointer = 0

        # 直到所有会议都处理完毕
        while start_pointer < L:
            # 如果有一个会议在 `start_pointer` 开始时已经结束
            if start_timings[start_pointer] >= end_timings[end_pointer]:
                # 释放一个房间并递增end_pointer。
                used_rooms -= 1
                end_pointer += 1

            # 无论房间是否空闲，我们都会这样做。
            # 如果一个房间是空闲的，那么 used_rooms+=1 将不会有任何效果。 used_rooms
            # 在这种情况下会保持不变。如果没有空闲的房间，则会增加已用房间数。
            used_rooms += 1
            start_pointer += 1

        return used_rooms
