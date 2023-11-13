# 307_Range_Sum_Query_-_Mutable
# https://leetcode.cn/problems/range-sum-query-mutable/description/


# 树状数组 太难了 完全看不懂证明

# 树状数组（Binary Indexed Tree，简称BIT或Fenwick Tree）是一种用于高效处理数组前缀和、区间和等问题的数据结构。它在某些特定场景下，比如频繁更新数组元素同时需要计算区间和的情况，比线段树更为简单和高效。

# ### 基本原理

# 1. **存储方式**：树状数组是基于数组实现的，但它通过特殊的方式组织数据，使得可以快速更新元素和计算前缀和。数组的每个元素不直接存储原数组的值，而是存储一定范围内的元素和。

# 2. **索引结构**：树状数组的索引从1开始，每个索引的二进制表示中最低位的1及其后面的位数定义了该索引覆盖的区间范围。例如，索引 `8 (1000)` 覆盖 `8` 个元素，而 `6 (0110)` 覆盖 `2` 个元素。

# 3. **更新操作**：当更新数组中的一个元素时，树状数组可以在对数时间内完成更新。更新涉及的节点数量大约是索引二进制表示中1的个数。

# 4. **查询操作**：同样地，树状数组可以在对数时间内计算任意前缀和。计算过程是通过跳跃索引实现的，每次跳到去掉最低位1的索引上。

# ### 应用场景

# - **动态前缀和**：在一系列数值更新后快速计算前缀和。
# - **区间和查询**：可以通过两次前缀和查询来计算任意区间和。
# - **动态逆序对计数**：在排序算法中计算逆序对数量。

# ### 实现

# 树状数组通常使用数组实现。关键操作包括：

# - **初始化**：创建一个数组，大小通常为原数组大小加一（因为索引从1开始）。
# - **更新**（`update`）：增加或减少数组中某一元素的值。
# - **查询前缀和**（`query`）：计算从数组开始到指定索引的所有元素的和。

# ### 示例

# 假设有一个数组 `a`，树状数组 `bit` 用来存储其前缀和。对于 `a` 中的每次更新，你会更新 `bit` 中相应的元素和区间和。当需要计算 `a[1]` 到 `a[i]` 的和时，你可以快速通过 `bit` 得到结果。

# ### 总结

# 树状数组是一种高效的数据结构，适用于需要频繁更新元素和计算前缀和的场景。它比其他类似的数据结构（如线段树）更简单，且在实际应用中常常更高效。它的主要缺点是只能处理前缀和类型的问题，不能像线段树那样灵活地处理更复杂的区间查询。

from typing import List

class NumArray:
    __slots__ = 'nums', 'tree'

# 设 delta=val−nums[index]，相当于把 index 的元素增加了这么多。然后把 nums[index] 更新成 val。
# 初始化 i=index+1（注意下标从 1 开始），这是第一个被更新的关键区间的右端点。
# 不断循环直到 i>n，这里 n 是 nums 的长度。
# 每次循环，把 tree[i] 增加 delta。
# 然后更新 i 为 i+lowbit(i)，即下一个被更新的关键区间的右端点。
    def __init__(self, nums: List[int]):
        n = len(nums)
        self.nums = [0] * n # 使 update 中算出的 delta = nums[i]
        self.tree = [0] * (n + 1)
        for i, x in enumerate(nums):
            self.update(i, x)
            
    def update(self, index: int, val: int) -> None:
        delta = val - self.nums[index]
        self.nums[index] = val
        i = index + 1
        while i < len(self.tree):
            self.tree[i] += delta
            i += i & -i            
    
    def prefixSum(self, i: int) -> int:
        s = 0
        while i:
            s += self.tree[i]
            i &= i - 1 # 同 i -= i & -1
        return s

    def sumRange(self, left: int, right: int) -> int:
        return self.prefixSum(right + 1) - self.prefixSum(left)



# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(index,val)
# param_2 = obj.sumRange(left,right)


# The line `__slots__ = 'nums', 'tree'` in Python is used within a class definition 
# to declare a fixed set of attributes that the instances of that class will have. 
# This serves a couple of significant purposes:

# 1. **Memory Efficiency**: When `__slots__` is defined, Python uses a more memory-efficient 
# representation for instances of the class. 
# Normally, Python objects use a dictionary to store their attributes, 
# which can be quite memory-intensive. With `__slots__`, 
# Python uses a more compact array-like structure which can significantly reduce 
# the memory footprint of each instance, especially if you are creating a large number of them.

# 2. **Attribute Restriction**: It restricts the class to only allow the 
# creation of attributes specified in `__slots__`. 
# In other words, instances of the class cannot have any attributes outside 
# of those listed in `__slots__`. This can help prevent bugs due to 
# typos or unintended attribute creation.

# In your specific example, `__slots__ = 'nums', 'tree'`, the class is restricted to 
# have only two attributes, `nums` and `tree`. Here's a breakdown:

# - `'nums'`: This would be an attribute name, and instances of the class can have an attribute called `nums`.
# - `'tree'`: Similarly, instances can also have an attribute called `tree`.

# The effect of using `__slots__` is most notable in classes that are expected to 
# have many instances, as the memory savings can be substantial.

# ### Points to Note:

# - When using `__slots__`, you cannot use some Python features like 
#   weak references or multiple inheritance 
#   unless you explicitly include them in `__slots__`.
# - `__slots__` is particularly useful in large-scale systems or 
#   frameworks where memory efficiency is crucial.
# - If a class inherits from another class without `__slots__`, 
#   the benefits of memory efficiency are lost. 
#   The derived class will revert to using a dictionary for attribute storage.
# - `__slots__` is not always necessary; for small-scale applications or classes with few instances, 
#   the benefit might not be significant.

# In summary, `__slots__ = 'nums', 'tree'` in a Python class is a technique to 
# optimize memory usage and prevent the dynamic creation of 
# additional attributes outside of `nums` and `tree`.

# 总结来说，在Python类中的__slots__ = 'nums', 'tree'是一种优化内存使用和防止在nums和tree之外动态创建额外属性的技术。


class NumArray:
    __slots__ = 'nums', 'tree'

    def __init__(self, nums: List[int]):
        n = len(nums)
        self.nums = [0] * n  # 使 update 中算出的 delta = nums[i]
        self.tree = [0] * (n + 1)
        for i, x in enumerate(nums):
            self.update(i, x)

    def update(self, index: int, val: int) -> None:
        delta = val - self.nums[index]
        self.nums[index] = val
        i = index + 1
        while i < len(self.tree):
            self.tree[i] += delta
            i += i & -i

    def prefixSum(self, i: int) -> int:
        s = 0
        while i:
            s += self.tree[i]
            i &= i - 1  # i -= i & -i 的另一种写法
        return s

    def sumRange(self, left: int, right: int) -> int:
        return self.prefixSum(right + 1) - self.prefixSum(left)
    
    
# 继续优化
class NumArray:
    __slots__ = 'nums', 'tree'

    def __init__(self, nums: List[int]):
        n = len(nums)
        tree = [0] * (n + 1)
        for i, x in enumerate(nums, 1):  # i 从 1 开始
            tree[i] += x
            nxt = i + (i & -i)  # 下一个关键区间的右端点
            if nxt <= n:
                tree[nxt] += tree[i]
        self.nums = nums
        self.tree = tree

    def update(self, index: int, val: int) -> None:
        delta = val - self.nums[index]
        self.nums[index] = val
        i = index + 1
        while i < len(self.tree):
            self.tree[i] += delta
            i += i & -i

    def prefixSum(self, i: int) -> int:
        s = 0
        while i:
            s += self.tree[i]
            i &= i - 1  # i -= i & -i 的另一种写法
        return s

    def sumRange(self, left: int, right: int) -> int:
        return self.prefixSum(right + 1) - self.prefixSum(left)
