# 146_LRU_Cache
# https://leetcode.cn/problems/lru-cache/description/

# 请你设计并实现一个满足  LRU (最近最少使用) 缓存 约束的数据结构。
# 实现 LRUCache 类：
# LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存

# int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。

# void put(int key, int value) 如果关键字 key 已经存在，则变更其数据值 value ；

# 如果不存在，则向缓存中插入该组 key-value 。

# 如果插入操作导致关键字数量超过 capacity ，则应该 逐出 最久未使用的关键字。

# 函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。

# 提示：
# 1 <= capacity <= 3000
# 0 <= key <= 10000
# 0 <= value <= 105
# 最多调用 2 * 105 次 get 和 put

class LRUCache:

    def __init__(self, capacity: int):
        self.map = {}
        self.capacity = capacity

    def get(self, key: int) -> int:
        if not self.map.get(key):
            return -1
        else:
            v, _ = self.map.get(key)
            self.map = {(k, (v, c+1)) for (k, (v, c)) in self.map}
            self.map[key] = (v, 0)
            return v
        
    def put(self, key: int, value: int) -> None:
        #
        if not self.map.get(key):
            if len(self.map) > self.capacity:
                max_key = max(self.map, key=self.map.get[1])
                del self.map[max_key]
                self.map = {(k, (v, c+1)) for (k, (v, c)) in self.map}
                self.map[key] = (value, 0)
            else:
                self.map = {(k, (v, c+1)) for (k, (v, c)) in self.map}
                self.map[key] = (value, 0)
                
        
        


# Your LRUCache object will be instantiated and called as such:
capacity=3000
obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)