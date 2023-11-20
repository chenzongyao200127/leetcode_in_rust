# 622_Design_Circular_Queue
# https://leetcode.cn/problems/design-circular-queue/

class Node:
    def __init__(self, v: int):
        self.pre = None
        self.nxt = None
        self.val = v

class MyCircularQueue:

    def __init__(self, k: int):
        self.cnt = 0
        self.capacity = k
        self.dummy = Node(0)  # Initialize with a dummy node
        self.dummy.nxt = self.dummy
        self.dummy.pre = self.dummy

    def enQueue(self, value: int) -> bool:
        if self.cnt == self.capacity:
            return False
        
        node = Node(value)
        last_node = self.dummy.pre
        last_node.nxt = node
        node.pre = last_node
        node.nxt = self.dummy
        self.dummy.pre = node
        
        self.cnt += 1
        
        return True

    def deQueue(self) -> bool:
        if self.cnt == 0:
            return False
        
        second_node = self.dummy.nxt.nxt
        self.dummy.nxt = second_node
        second_node.pre = self.dummy
        
        self.cnt -= 1
         
        return True

    def Front(self) -> int:
        if self.cnt == 0:
            return -1
        return self.dummy.nxt.val

    def Rear(self) -> int:
        if self.cnt == 0:
            return -1
        return self.dummy.pre.val

    def isEmpty(self) -> bool:
        return self.cnt == 0

    def isFull(self) -> bool:
        return self.cnt == self.capacity




# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue(k)
# param_1 = obj.enQueue(value)
# param_2 = obj.deQueue()
# param_3 = obj.Front()
# param_4 = obj.Rear()
# param_5 = obj.isEmpty()
# param_6 = obj.isFull()