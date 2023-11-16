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

# 超时
class LRUCache:

    def __init__(self, capacity: int):
        self.map = {}
        self.capacity = capacity

    def get(self, key: int) -> int:
        if key not in self.map:
            return -1
        else:
            v, _ = self.map[key]
            
            # Increment count for all other keys
            for k in self.map:
                self.map[k] = (self.map[k][0], self.map[k][1] + 1)
                
            # Reset count for accessed key
            self.map[key] = (v, 0)
            return v

    def put(self, key: int, value: int) -> None:
        if key in self.map:
            # Reset count for existing key and increment others
            for k in self.map:
                self.map[k] = (self.map[k][0], self.map[k][1] + 1)
                
            self.map[key] = (value, 0)
        else:
            if len(self.map) >= self.capacity:
                # Find and remove the least recently used item
                max_key = max(self.map, key=lambda k: self.map[k][1])
                del self.map[max_key]

            # Increment count for all keys
            for k in self.map:
                self.map[k] = (self.map[k][0], self.map[k][1] + 1)
                
            # Add new item with count 0
            self.map[key] = (value, 0)

        
        


# Your LRUCache object will be instantiated and called as such:
capacity=3000
obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)

from typing import Optional

class Node:
    # 提高访问属性的速度，并节省内存
    __slots__ = 'prev', 'next', 'key', 'value'

    def __init__(self, key=0, value=0):
        self.key = key
        self.value = value
        

from typing import Optional

class Node:
    # 提高访问属性的速度，并节省内存
    __slots__ = 'prev', 'next', 'key', 'value'

    def __init__(self, key=0, value=0):
        self.key = key
        self.value = value
        

class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.dummy = Node()             # 哨兵节点
        self.dummy.prev = self.dummy
        self.dummy.next = self.dummy
        self.key_to_node = dict()

    def get_node(self, key: int) -> Optional[Node]:
        if key not in self.key_to_node:
            return None
        node = self.key_to_node[key]
        self.remove(node)
        self.push_front(node)
        return node
    
    def get(self, key: int) -> int:
        node = self.get_node(key)
        return node.value if node else -1


    def put(self, key: int, value: int) -> None:
        node = self.get_node(key)
        if node:
            node.value = value
            return
        self.key_to_node[key] = node = Node(key, value)  # 新书
        self.push_front(node)
        if len(self.key_to_node) > self.capacity:
            back_node = self.dummy.prev
            del self.key_to_node[back_node.key]
            self.remove(back_node)
        
    # 删除一个节点（抽出一本书）
    def remove(self, x: Node) -> None:
        x.prev.next = x.next
        x.next.prev = x.prev

    # 在链表头添加一个节点（把一本书放在最上面）
    def push_front(self, x: Node) -> None:
        x.prev = self.dummy
        x.next = self.dummy.next
        x.prev.next = x
        x.next.prev = x
        

# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)


from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity: int):
        """ Initialize the LRU cache with a given capacity. """
        self.capacity = capacity
        self.od = OrderedDict()

    def get(self, key: int) -> int:
        """ Retrieve an item from the cache by key. Returns -1 if not found. """
        if key in self.od:
            self.od.move_to_end(key)
            return self.od[key]
        return -1

    def put(self, key: int, value: int):
        """ Put an item into the cache. If the cache exceeds its capacity, removes the least recently used item. """
        self.od[key] = value
        if len(self.od) > self.capacity:
            self.od.popitem(last=False)
        self.od.move_to_end(key)

# Example usage:
# cache = LRUCache(2)
# cache.put(1, 1)
# cache.put(2, 2)
# print(cache.get(1)) # returns 1
# cache.put(3, 3) # evicts key 2
# print(cache.get(2)) # returns -1 (not found)

### OrderedDict 
# `OrderedDict` is a subclass of the dictionary class in Python's `collections` module.
# Introduced in Python 2.7, it retains the order in which its elements are added, 
# a feature that was not available in the standard dictionary (`dict`) in earlier versions of Python. 
# Here's a detailed overview:

# ### Key Features of OrderedDict

# 1. **Order Preservation**: 
# The main feature of an `OrderedDict` is that it maintains the order of elements as they are inserted. 
# When you iterate over an `OrderedDict`, elements are returned in the order they were added.

# 2. **Equivalent to Regular Dictionary in Python 3.7+**: 
# Starting from Python 3.7, the built-in `dict` type also maintains insertion order by default. 
# However, `OrderedDict` is still relevant because it has some unique features.

# 3. **Equality Testing**: 
# An `OrderedDict` considers the order of elements when checking for equality. 
# Two `OrderedDict` objects are considered equal if their order and contents are the same. 
# This is in contrast to standard `dict`, where order is not considered in equality checks.

# ### Methods and Operations

# `OrderedDict` supports all methods and operations aWvailable in a regular dictionary, plus a few additional methods:

# - **`popitem(last=True)`**: This method removes and returns a `(key, value)` pair. 
# The pairs are returned in LIFO order if `last` is true or FIFO order if false.

# - **`move_to_end(key, last=True)`**: This method moves an existing key to either end of the `OrderedDict`. 
# If `last` is true (default), the item is moved to the right end. 
# If `last` is false, the item is moved to the beginning.

# - **`reversed()`**: In Python 3.8 and later, `OrderedDict` supports the `reversed()` 
# function to return a reverse iterator over the keys of the dictionary.

# ### Use Cases

# `OrderedDict` is particularly useful in scenarios where the order of elements is crucial. For example:
# - **LRU Caches**: As in your use case, `OrderedDict` is perfect for implementing LRU (Least Recently Used) caches.
# - **Maintaining Insertion Order**: In data processing where the order of entries is important.
# - **Coding Problems and Algorithms**: Where the order of elements affects the output.

# ### Example Usage

# Here's a simple example of using `OrderedDict`:

# ```python
from collections import OrderedDict

# Creating an ordered dictionary
od = OrderedDict()
od['a'] = 1
od['b'] = 2
od['c'] = 3

# Adding an item to the end
od['d'] = 4

# Moving 'b' to the end
od.move_to_end('b')

# Iterating over OrderedDict
for key, value in od.items():
    print(key, value)
# ```

# In this example, the iteration order will be `'a', 'c', 'd', 'b'`.

# ### Conclusion

# While `OrderedDict` is not as commonly used since Python 3.7 due to the standard dictionary 
# also maintaining insertion order, its additional methods and the guaranteed 
# order-preserving feature make it a valuable tool in certain Python programming scenarios.


class Node:
    # Using __slots__ for memory efficiency
    __slots__ = 'prev', 'next', 'key', 'value'

    def __init__(self, key=0, value=0):
        self.key = key
        self.value = value
        

class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.head = Node()  # Sentinel node to denote the start of the cache
        self.head.prev = self.head
        self.head.next = self.head
        self.key_to_node = dict()

    def _get_node(self, key: int) -> Node:
        """Retrieve a node from the cache based on the key."""
        if key not in self.key_to_node:
            return None
        node = self.key_to_node[key]
        self._remove(node)
        self._add_to_front(node)
        return node
    
    def get(self, key: int) -> int:
        """Retrieve a value from the cache based on the key."""
        node = self._get_node(key)
        return node.value if node else -1

    def put(self, key: int, value: int) -> None:
        """Add or update a key-value pair in the cache."""
        node = self._get_node(key)
        if node:
            node.value = value
        else:
            if len(self.key_to_node) >= self.capacity:
                # Remove the least recently used item if the cache is full
                oldest_node = self.head.prev
                del self.key_to_node[oldest_node.key]
                self._remove(oldest_node)

            node = Node(key, value)
            self.key_to_node[key] = node
            self._add_to_front(node)

    def _remove(self, node: Node) -> None:
        """Remove a node from the cache."""
        node.prev.next = node.next
        node.next.prev = node.prev

    def _add_to_front(self, node: Node) -> None:
        """Add a node to the front (most recent) position in the cache."""
        node.prev = self.head
        node.next = self.head.next
        self.head.next.prev = node
        self.head.next = node

# Example Usage:
# cache = LRUCache(2)
# cache.put(1, 1)
# cache.put(2, 2)
# print(cache.get(1))  # returns 1
# cache.put(3, 3)  # L
