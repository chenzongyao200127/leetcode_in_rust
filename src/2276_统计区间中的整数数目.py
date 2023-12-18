# 2276_统计区间中的整数数目
# https://leetcode.cn/problems/count-integers-in-intervals/description/?envType=daily-question&envId=2023-12-16

from sortedcontainers import SortedDict

class CountIntervals:
    def __init__(self):
        # Initialize a SortedDict to store intervals. 
        # SortedDict is used to maintain the intervals in sorted order.
        self.mp = SortedDict()
        # Initialize a counter to keep track of the total count of unique elements in all intervals.
        self.cnt = 0

    def add(self, left: int, right: int) -> None:
        # Find the position in the sorted dictionary where 'right' would fit.
        interval_index = self.mp.bisect_right(right)

        # Adjust the index if it's not the first element.
        if interval_index != 0:
            interval_index -= 1

        # Merge overlapping intervals and adjust the total count accordingly.
        while interval_index < len(self.mp) and self.mp.keys()[interval_index] <= right \
                                            and self.mp.values()[interval_index] >= left:
            l, r = self.mp.items()[interval_index]
            left = min(left, l)
            right = max(right, r)
            # Decrease the count by the size of the current interval as it will be merged.
            self.cnt -= r - l + 1
            # Remove the current interval as it is being merged into the new interval.
            self.mp.popitem(interval_index)

            # Recalculate the position for the merged interval.
            interval_index = self.mp.bisect_right(right)
            if interval_index != 0:
                interval_index -= 1

        # Add the new/merged interval's size to the total count.
        self.cnt += right - left + 1
        # Store the new/merged interval in the sorted dictionary.
        self.mp[left] = right

    def count(self) -> int:
        # Return the total count of unique elements in all intervals.
        return self.cnt


# `SortedDict` is a data structure provided by the `sortedcontainers` module in Python. It is a sorted dictionary that maintains its keys in sorted order. This is particularly useful when you need the functionalities of a dictionary along with the ability to efficiently retrieve the keys or values in a sorted manner. Here's a detailed introduction to its usage:

# ### Installation
# Before you can use `SortedDict`, you need to install the `sortedcontainers` module:

# ```bash
# pip install sortedcontainers
# ```

# ### Importing
# After installation, you can import `SortedDict` like this:

# ```python
# from sortedcontainers import SortedDict
# ```

# ### Creating a SortedDict
# You can create a `SortedDict` much like a regular dictionary:

# ```python
# sd = SortedDict()
# ```

# Or you can initialize it with key-value pairs:

# ```python
# sd = SortedDict({key1: value1, key2: value2, ...})
# ```

# ### Adding and Accessing Elements
# - **Adding:** You can add elements to a `SortedDict` just like a regular dictionary.

#   ```python
#   sd[key] = value
#   ```

# - **Accessing:** Access elements using keys like a standard dictionary.

#   ```python
#   value = sd[key]
#   ```

# ### Key Sorting
# The keys are automatically sorted, so when you iterate over the `SortedDict`, the keys are returned in sorted order:

# ```python
# for key in sd:
#     print(key, sd[key])
# ```

# ### Special Methods
# - **`bisect_left(key)` and `bisect_right(key)`**: These methods are used to find the insertion points for a key in the sorted keys.

#   ```python
#   index = sd.bisect_left(key)
#   index = sd.bisect_right(key)
#   ```

# - **`popitem(index=-1)`**: Removes and returns a `(key, value)` pair. The default is to remove the last item.

#   ```python
#   key, value = sd.popitem()  # Removes the last item
#   ```

# - **`peekitem(index=-1)`**: Returns a `(key, value)` pair without removing it. The default is the last item.

#   ```python
#   key, value = sd.peekitem()  # Peeks at the last item
#   ```

# ### Performance
# `SortedDict` is designed to provide efficient, O(log n) operations for insertions, deletions, and lookups. This makes it suitable for scenarios where you need a dictionary but also require maintaining the keys in a sorted order.

# ### Use Cases
# `SortedDict` is particularly useful in scenarios where:
# - The order of keys matters, and you frequently need to retrieve the smallest or largest key.
# - You need to perform range queries or find keys that are closest to a certain value.

# In summary, `SortedDict` from the `sortedcontainers` module offers a combination of the flexibility of a dictionary with the order maintenance of a sorted list. It's a valuable tool for Python developers needing sorted dictionary functionalities.