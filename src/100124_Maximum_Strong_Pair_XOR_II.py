# 100124_Maximum_Strong_Pair_XOR_II
# https://leetcode.cn/problems/maximum-strong-pair-xor-ii/description/

from typing import List


class Solution:
    def maximumStrongPairXor(self, nums: List[int]) -> int:
        nums.sort()
        ans = mask = 0
        high_bit = nums[-1].bit_length() - 1
        for i in range(high_bit, -1, -1):  # 从最高位开始枚举
            mask |= 1 << i
            new_ans = ans | (1 << i)  # 这个比特位可以是 1 吗？
            d = {}
            for y in nums:
                mask_y = y & mask  # 低于 i 的比特位置为 0
                if new_ans ^ mask_y in d and d[new_ans ^ mask_y] * 2 >= y:
                    ans = new_ans  # 这个比特位可以是 1
                    break
                d[mask_y] = y

        return ans


class Node:
    __slots__ = 'children', 'cnt'

    def __init__(self):
        self.children = [None, None]
        self.cnt = 0  # 子树大小


class Trie:
    HIGH_BIT = 19

    def __init__(self):
        self.root = Node()

    # 添加 val
    def insert(self, val: int) -> None:
        cur = self.root
        for i in range(Trie.HIGH_BIT, -1, -1):
            bit = (val >> i) & 1
            if cur.children[bit] is None:
                cur.children[bit] = Node()
            cur = cur.children[bit]
            cur.cnt += 1  # 维护子树大小
        return cur

    # 删除 val，但不删除节点
    # 要求 val 必须在 trie 中
    def remove(self, val: int) -> None:
        cur = self.root
        for i in range(Trie.HIGH_BIT, -1, -1):
            cur = cur.children[(val >> i) & 1]
            cur.cnt -= 1  # 维护子树大小
        return cur

    # 返回 val 与 trie 中一个元素的最大异或和
    # 要求 trie 中至少有一个元素
    def max_xor(self, val: int) -> int:
        cur = self.root
        ans = 0
        for i in range(Trie.HIGH_BIT, -1, -1):
            bit = (val >> i) & 1
            # 如果 cur.children[bit^1].cnt == 0，视作空节点
            if cur.children[bit ^ 1] and cur.children[bit ^ 1].cnt:
                ans |= 1 << i
                bit ^= 1
            cur = cur.children[bit]
        return ans


class Solution:
    def maximumStrongPairXor(self, nums: List[int]) -> int:
        nums.sort()
        t = Trie()
        ans = left = 0
        for y in nums:
            t.insert(y)
            while nums[left] * 2 < y:
                t.remove(nums[left])
                left += 1
            ans = max(ans, t.max_xor(y))
        return ans


# 0-1 trie 板子
class Node:
    # Optimize memory usage with __slots__
    __slots__ = 'children', 'cnt'

    def __init__(self):
        self.children = [None, None]  # Binary trie nodes
        self.cnt = 0  # Count of numbers in the subtree


class Trie:
    # Define the highest bit we will consider (0-indexed, 19 for 32-bit integers)
    HIGH_BIT = 19

    def __init__(self):
        self.root = Node()

    # Insert a value into the trie
    def insert(self, val: int) -> None:
        cur = self.root
        for i in range(Trie.HIGH_BIT, -1, -1):
            # Extract the i-th bit from val
            bit = (val >> i) & 1
            # Create a new node if necessary
            if cur.children[bit] is None:
                cur.children[bit] = Node()
            cur = cur.children[bit]
            cur.cnt += 1  # Increment the count of numbers in the subtree
        return cur

    # Remove a value from the trie (assumes the value exists)
    def remove(self, val: int) -> None:
        cur = self.root
        for i in range(Trie.HIGH_BIT, -1, -1):
            # Navigate to the correct child based on the i-th bit of val
            cur = cur.children[(val >> i) & 1]
            cur.cnt -= 1  # Decrement the count of numbers in the subtree
        return cur

    # Find the maximum XOR of val with any number in the trie
    # Assumes that the trie is not empty
    def max_xor(self, val: int) -> int:
        cur = self.root
        ans = 0
        for i in range(Trie.HIGH_BIT, -1, -1):
            # Extract the i-th bit from val
            bit = (val >> i) & 1
            # Try to go in the opposite direction of bit to maximize XOR
            # Only go if the node exists and is not empty
            if cur.children[bit ^ 1] and cur.children[bit ^ 1].cnt:
                ans |= 1 << i  # Set the i-th bit in the answer
                bit ^= 1  # Flip the bit to go in the opposite direction
            cur = cur.children[bit]
        return ans
