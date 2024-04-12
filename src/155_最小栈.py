# 155_最小栈
# https://leetcode.cn/problems/min-stack/description/?envType=study-plan-v2&envId=top-100-liked

from collections import deque


class MinStack:
    def __init__(self):
        self.stk = deque()

    def push(self, val: int) -> None:
        if self.stk:
            cur_min = self.getMin()
            new_min = min(cur_min, val)
            self.stk.appendleft((val, new_min))
        else:
            self.stk.appendleft((val, val))

    def pop(self) -> None:
        self.stk.popleft()

    def top(self) -> int:
        return self.stk[0][0]

    def getMin(self) -> int:
        return self.stk[0][1]

        # Your MinStack object will be instantiated and called as such:
        # obj = MinStack()
        # obj.push(val)
        # obj.pop()
        # param_3 = obj.top()
        # param_4 = obj.getMin()
