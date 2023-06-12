# 401. Binary Watch
# https://leetcode.cn/problems/binary-watch/

# 方法一：枚举时分
class Solution:
    def readBinaryWatch(self, turnedOn: int) -> List[str]:
        ans = list()
        for h in range(12):
            for m in range(60):
                if bin(h).count("1") + bin(m).count("1") == turnedOn:
                    ans.append(f"{h}:{m:02d}")
        return ans

# 方法二：二进制枚举
class Solution:
    def readBinaryWatch(self, turnedOn: int) -> List[str]:
        ans = list()
        for i in range(1024):
            h, m = i >> 6, i & 0x3f   # 用位运算取出高 4 位和低 6 位
            if h < 12 and m < 60 and bin(i).count("1") == turnedOn:
                ans.append(f"{h}:{m:02d}")
        return ans


class Solution:
    def __init__(self):
        self.nums = [1, 2, 4, 8, 1, 2, 4, 8, 16, 32]

    def readBinaryWatch(self, num):
        def dfs(num, step, start, visited):
            if step == num:
                hour, minute = calc_time(visited)
                if 0 <= hour <= 11 and 0 <= minute <= 59:
                    result_all.append(f"{hour}:{minute:02d}")
                return
            for i in range(start, len(self.nums)):
                visited[i] = 1
                dfs(num, step + 1, i + 1, visited)
                visited[i] = 0

        def calc_time(visited):
            hour = sum(self.nums[i] for i in range(4) if visited[i])
            minute = sum(self.nums[i] for i in range(4, len(visited)) if visited[i])
            return hour, minute

        result_all = []
        dfs(num, 0, 0, [0] * len(self.nums))
        return result_all