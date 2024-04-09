# 146_LRU_缓存

from typing import Optional
from collections import OrderedDict
from collections import deque

# 通过但是超时


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.maps = dict()
        self.order = deque()

    def get(self, key: int) -> int:
        # 超时瓶颈
        if key in self.maps:
            value = self.maps[key]
            self.mark_as_freq(key)
            return value
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        # 超时瓶颈
        if key in self.maps:
            # Update the value for existing keys without checking the capacity
            self.maps[key] = value
            self.mark_as_freq(key)
        else:
            # Only check the capacity if it's a new key
            if len(self.maps) == self.capacity:
                # Remove the least recently used item
                old_key = self.order.pop()
                del self.maps[old_key]
            self.maps[key] = value
            self.order.appendleft(key)

    def mark_as_freq(self, key):
        # Move the key to the front (left side) of the deque to show it was recently accessed
        self.order.remove(key)
        self.order.appendleft(key)

# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)

# 使用Python OrderedDict()
# 有序字典


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


cache = LRUCache(2)
cache.put(1, 1)
cache.put(2, 2)
print(cache.get(1))  # returns 1
cache.put(3, 3)  # L


# Python 双链表实现，比较复杂
class Node:
    __slots__ = 'prev', 'next', 'key', 'value'

    def __init__(self, key=0, value=0):
        self.key = key
        self.value = value


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.dummy = Node()             # Sentinel node for ease of use
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
        if key in self.key_to_node:
            node = self.get_node(key)
            node.value = value
        else:
            if len(self.key_to_node) == self.capacity:
                # Remove the least recently used node before adding a new one
                back_node = self.dummy.prev
                del self.key_to_node[back_node.key]
                self.remove(back_node)
            node = Node(key, value)
            self.key_to_node[key] = node
            self.push_front(node)

    def remove(self, node: Node) -> None:  # Corrected type hint
        node.prev.next = node.next
        node.next.prev = node.prev

    def push_front(self, node: Node) -> None:
        node.prev = self.dummy
        node.next = self.dummy.next
        self.dummy.next.prev = node
        self.dummy.next = node  # Corrected order of assignments
