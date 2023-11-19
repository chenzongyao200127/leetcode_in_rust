# 100110_找到_Alice_和_Bob_可以相遇的建筑
# https://leetcode.cn/problems/find-building-where-alice-and-bob-can-meet/description/

# 堆（Heap）是一种特殊的完全二叉树，广泛用于实现优先队列。它具有以下特点：

# 1. **完全二叉树**：除了最后一层外，每一层都是完全填满的，且最后一层的节点尽可能地集中在左侧。
# 2. **堆属性**：堆可以是最大堆或最小堆。在最大堆中，每个节点的值都大于或等于其子节点的值；在最小堆中，每个节点的值都小于或等于其子节点的值。

# ### 常见的实现方式

# 堆通常通过数组来实现，其中数组的索引有特殊的含义：
# - 对于数组中的任意一个元素，假设其索引为 `i`，则：
#   - 它的父节点的索引是 `(i-1)/2`（向下取整）。
#   - 它的左子节点的索引是 `2*i + 1`。
#   - 它的右子节点的索引是 `2*i + 2`。

# ### 基本操作

# 1. **插入（Heap Insertion）**：
#    - 将新元素添加到数组的末尾。
#    - 将这个新元素向上移动到正确的位置，以维护堆的属性。这通常通过一个称为“上浮”（Heapify Up）的过程实现。

# 2. **删除（Heap Deletion）**：
#    - 在最大堆中，通常删除根节点，因为它是最大的元素。
#    - 将数组的最后一个元素移动到根节点的位置。
#    - 将这个元素向下移动到正确的位置，以维护堆的属性。这通常通过一个称为“下沉”（Heapify Down）的过程实现。

# 3. **构建堆（Heapify）**：
#    - 将一个无序的数组转换成一个堆。
#    - 从最后一个非叶子节点开始，对每个节点进行“下沉”操作，直到根节点。

# ### 应用

# - **优先队列**：堆是实现优先队列的一种有效方式，特别是当需要快速访问最大或最小元素时。
# - **堆排序**：通过堆来实现的排序算法，可以实现 O(n log n) 的时间复杂度。
# - **图算法**：如 Dijkstra 的最短路径算法和 Prim 的最小生成树算法，常使用堆来优化性能。

# 总的来说，堆是一种非常高效的数据结构，尤其适用于需要频繁访问或更新其最大或最小元素的场景。

# 要用 Python 实现一个堆，通常可以通过编写一个类来封装相关的操作。
# 这里我将为你展示一个简单的最小堆的实现。最小堆中，每个父节点的值都小于或等于其子节点的值。基本操作包括插入（上浮）、删除（下沉）、以及构建堆。

class MinHeap:
    def __init__(self):
        self.heap = []

    def parent(self, i):
        return (i - 1) // 2

    def left_child(self, i):
        return 2 * i + 1

    def right_child(self, i):
        return 2 * i + 2

    def has_parent(self, i):
        return self.parent(i) >= 0

    def has_left_child(self, i):
        return self.left_child(i) < len(self.heap)

    def has_right_child(self, i):
        return self.right_child(i) < len(self.heap)

    def swap(self, i, j):
        self.heap[i], self.heap[j] = self.heap[j], self.heap[i]

    def insert(self, key):
        self.heap.append(key)
        self.heapify_up(len(self.heap) - 1)

    def heapify_up(self, i):
        while self.has_parent(i) and self.heap[i] < self.heap[self.parent(i)]:
            self.swap(i, self.parent(i))
            i = self.parent(i)

    def remove_min(self):
        if len(self.heap) == 0:
            raise Exception("Heap is empty")
        min_item = self.heap[0]
        self.heap[0] = self.heap.pop()
        self.heapify_down(0)
        return min_item

    def heapify_down(self, i):
        while self.has_left_child(i):
            smaller_child_index = self.left_child(i)
            if self.has_right_child(i) and self.heap[self.right_child(i)] < self.heap[smaller_child_index]:
                smaller_child_index = self.right_child(i)

            if self.heap[i] < self.heap[smaller_child_index]:
                break
            else:
                self.swap(i, smaller_child_index)
            i = smaller_child_index

    def build_heap(self, arr):
        self.heap = arr
        for i in range((len(arr) // 2) - 1, -1, -1):
            self.heapify_down(i)

# 这个 `MinHeap` 类提供了插入、删除最小元素、上浮、下沉和构建堆的基本功能。你可以使用这个类来创建一个最小堆，并对其进行操作。
# 例如，构建堆的操作是将一个现有的数组转换成一个最小堆。

# 请注意，这个实现是一个基本的示例，可能需要根据具体的应用场景进行调整或优化。




# Python 的标准库中的 `heapq` 模块提供了对堆的支持，特别是最小堆。这个模块包括一些重要的函数来管理堆，下面是其中一些最常用的函数：

# 1. **`heappush(heap, item)`**：
#    - 将元素 `item` 添加到堆 `heap` 中。这会保持堆的不变性。
# 2. **`heappop(heap)`**：
#    - 从堆中弹出并返回最小的元素。如果堆为空，则抛出 `IndexError`。
# 3. **`heapify(x)`**：
#    - 将列表 `x` 转换成堆。这在列表已经有所有元素，但还不是堆时非常有用。
# 4. **`heapreplace(heap, item)`**：
#    - 弹出并返回堆中的最小元素，然后将新的 `item` 添加到堆中。这个操作比先调用 `heappop()` 后再调用 `heappush()` 更有效率。
# 5. **`nlargest(n, iterable[, key])` 和 `nsmallest(n, iterable[, key])`**：
#    - 分别返回数据集中最大和最小的 `n` 个元素。这些函数使用堆算法实现，对于找到相对较小的 `n` 个元素时非常有效。

# 下面是一个使用 `heapq` 的简单示例，展示了如何创建一个堆并进行基本操作：


import heapq

# 创建一个空堆
heap = []

# 向堆中添加元素
heapq.heappush(heap, 10)
heapq.heappush(heap, 1)
heapq.heappush(heap, 5)

# 从堆中弹出最小的元素
min_item = heapq.heappop(heap)
print("The minimum item is:", min_item)

# 转换列表为堆
arr = [3, 1, 4, 1, 5, 9, 2, 6]
heapq.heapify(arr)
print("The heapified list is:", arr)

# 获取最大的3个元素
largest = heapq.nlargest(3, arr)
print("The three largest numbers are:", largest)

# 获取最小的3个元素
smallest = heapq.nsmallest(3, arr)
print("The three smallest numbers are:", smallest)

# 这个代码展示了如何使用 `heapq` 模块的各种函数来操作堆。
# 由于 Python 的堆默认是最小堆，所以 `heappop()` 总是返回最小的元素。


# 堆排序（Heap Sort）是一种基于比较的排序算法，使用堆这种数据结构来实现。
# 它的主要优势是在最坏、最好和平均情况下都能提供 O(n log n) 的时间复杂度，且不需要额外的内存空间。堆排序过程可以分为两个主要阶段：

# 1. **构建堆（Heapify）**：
# 将无序的输入数组转换成一个最大堆（或最小堆）。
# 在最大堆中，父节点的值总是大于或等于其子节点的值，这样最大的元素总是位于堆的顶部。

# 2. **排序**：
# 一旦堆被构建完成，最大元素（位于数组的首位）就是整个数组中的最大值。
# 通过将其与数组的最后一个元素交换，并将数组大小减少1，可以将最大元素放到其最终位置。
# 然后重新调整（或"heapify"）剩余的元素以保持最大堆的属性，再次将最大元素移至数组的最后。重复这个过程直到所有元素都被放置在它们的最终位置。

# ### 堆排序的步骤

# 假设我们正在对一个数组进行排序：

# 1. **构建最大堆**：从最后一个非叶子节点开始，对每个节点执行下沉操作，直到达到根节点，以此构建最大堆。

# 2. **进行排序**：
#    - 将堆的根节点（最大值）与最后一个元素交换，将最大值放到其最终位置。
#    - 将剩余的 n-1 个元素重新构建成最大堆。
#    - 重复这个过程，每次都减少堆的大小，直到堆的大小为 1。

# ### 复杂度和性能

# - **时间复杂度**：堆排序的时间复杂度为 O(n log n)，这是因为构建堆的过程是 O(n)，并且每次调整堆的过程是 O(log n)，需要调整 n-1 次。
# - **空间复杂度**：由于堆排序可以在原数组上进行，因此它是一个原地排序算法，空间复杂度为 O(1)。

# ### 应用场景

# 堆排序适用于那些不需要稳定排序且对时间复杂度有严格要求的场景。
# 然而，由于其较差的缓存局部性和递归调用的开销，在实际应用中，它通常不如快速排序或归并排序流行。
# 但是，它在构建优先队列和进行堆选择等特定场景下非常有用。

# 现代编程语言中的内置排序函数通常采用高效且适用于多种数据类型和数据分布的算法。
# 下面是一些流行编程语言及其内置排序函数的实现方式：

# 1. **Python** (`sorted()` 和 `list.sort()` 方法)：
#    - Python 使用的是 Timsort 算法，这是一种混合排序算法，主要结合了归并排序和插入排序的技术。
#      它被设计为在各种类型的实际数据上表现得非常有效，特别是对于部分有序的数据集。
#    - Timsort 首先将列表分成小块（称为“runs”），然后对每个小块使用插入排序，接着将这些小块合并成更大的块，最后用归并排序来完成整个数组的排序。

# 2. **Java** (`Collections.sort()` 和 `Arrays.sort()`)：
#    - 在 Java 7 及以后的版本中，对于原始数据类型，`Arrays.sort()` 使用了双轴快速排序（Dual-Pivot Quicksort）算法。
#    - 对于对象数组，`Arrays.sort()` 使用的是 Timsort 算法，与 Python 类似。

# 3. **C++** (标准模板库中的 `sort()`)：
#    - C++ 标准库的 `sort()` 函数通常实现为一种快速排序的变体，但标准并未明确指定具体算法。
#      实际上，很多 C++ 标准库实现会在快速排序、堆排序和插入排序之间根据数据的特性动态选择最适合的算法。
#    - 对于较小的数据集，很多实现会使用插入排序，因为它在小数组上表现良好。

# 4. **JavaScript** (`.sort()` 方法)：
#    - JavaScript 引擎通常使用一种或多种排序算法的组合来实现 `.sort()` 方法。
#      例如，V8 引擎（用于 Chrome 和 Node.js）曾使用快速排序和插入排序的组合，但在更近的版本中转向了 Timsort。

# 在 Rust 中，标准库的排序功能是通过 `sort` 和 `sort_unstable` 方法提供的，这些方法是 `Vec<T>` 类型的一部分。
# Rust 的排序算法选择反映了该语言对性能和安全性的总体重视。

# 1. **`sort` 方法**：
#    - `sort` 方法在 Rust 中默认使用的是 TimSort 算法，这是一种混合型的排序算法，结合了归并排序和插入排序的优点。
#    - TimSort 特别适合处理部分有序的数据集，能够在多种情况下提供出色的性能。它也是一个稳定的排序算法，意味着相等元素的相对顺序会被保留。

# 2. **`sort_unstable` 方法**：
#    - `sort_unstable` 方法通常使用一种快速排序的变体，这种快速排序的实现是非稳定的，但在某些情况下可能比 `sort` 方法更快。
#    - 非稳定排序不保证保持相等元素的原始顺序，但由于减少了这种保证，它可以在某些情况下提供更好的性能。

# Rust 的这种方法体现了其对于性能和实用性的综合考虑。`sort` 方法提供了一种稳定且在大多数情况下都非常高效的排序，而 `sort_unstable` 方法则为特定场景提供了一个可能更快的替代方案。用户可以根据自己的需要选择合适的方法，这与 Rust 提供选择和控制权给程序员的总体哲学相符。

# 这些语言选择特定的排序算法是基于它们在不同类型的数据集上的平均性能和稳定性。例如，Timsort 非常适合于处理现实世界中常见的部分有序数据，而快速排序的变体在大多数情况下都非常高效。每种语言的标准库都力求找到这样一种平衡点：在最广泛的使用场景下提供最佳的性能。

# ### 归并排序（Merge Sort）

# 归并排序是一种有效的、基于比较的、分而治之类型的排序算法。由约翰·冯·诺伊曼在1945年提出。

# #### 工作原理：

# 1. **分解**：递归地将当前序列分成两个子序列，直到每个子序列只有一个元素或为空。
# 2. **合并**：递归地将两个子序列合并成一个有序序列，直到最终只剩下一个排序完成的序列。

# #### 步骤：

# 1. 将序列逐级分解成较小的半子序列，直到每个子序列只包含一个元素。
# 2. 然后逐级合并子序列，以产生排序好的子序列，最终合并成一个完全排序的序列。

# #### 复杂度：

# - **时间复杂度**：平均和最坏情况下均为 O(n log n)。
# - **空间复杂度**：由于需要额外的存储空间来合并序列，归并排序的空间复杂度为 O(n)。

# #### 优点与缺点：

# - **优点**：稳定的排序算法，对于大数据集合效率高，分布式处理时表现良好。
# - **缺点**：相较于其他排序算法，如快速排序，它需要更多的内存空间。


def merge_sort(arr):
    if len(arr) > 1:
        mid = len(arr) // 2  # 找到中点，分割数组
        L = arr[:mid]  # 分割左半部分
        R = arr[mid:]  # 分割右半部分

        merge_sort(L)  # 递归排序左半部分
        merge_sort(R)  # 递归排序右半部分

        i = j = k = 0

        # 合并两个有序数组
        while i < len(L) and j < len(R):
            if L[i] < R[j]:
                arr[k] = L[i]
                i += 1
            else:
                arr[k] = R[j]
                j += 1
            k += 1

        # 检查是否有剩余元素
        while i < len(L):
            arr[k] = L[i]
            i += 1
            k += 1

        while j < len(R):
            arr[k] = R[j]
            j += 1
            k += 1

# 测试归并排序
arr = [12, 11, 13, 5, 6, 7]
merge_sort(arr)
print("Sorted array is:", arr)



# ### 插入排序（Insertion Sort）

# 插入排序是一种简单直观的排序算法，它的工作原理类似于整理手中的扑克牌。

# #### 工作原理：

# 1. 从数组第二个元素开始，将当前元素与之前的元素逐一比较。
# 2. 如果当前元素（待插入元素）小于已排序的元素，则将已排序元素向右移动，为当前元素腾出位置。
# 3. 重复这个过程，直到找到当前元素的适当位置并插入。

# #### 步骤：

# 1. 假设第一个元素已经排序，从第二个元素开始。
# 2. 取出下一个元素，在已排序的元素序列中从后向前扫描。
# 3. 如果已排序的元素大于新元素，将该元素移到下一位置。
# 4. 重复步骤3，直到找到已排序的元素小于或等于新元素的位置。
# 5. 将新元素插入到该位置后。
# 6. 重复步骤2~5。

# #### 复杂度：

# - **时间复杂度**：平均和最坏情况下为 O(n²)，最好情况为 O(n)（当输入数组已经是排序好的）。
# - **空间复杂度**：O(1)，因为它是原地排序算法。

# #### 优点与缺点：

# - **优点**：简单，对于较小的数据集或几乎已经排序好的数据集效率较高。
# - **缺点**：对于较大的数据集效率不高，算法的速度显著慢于更复杂的算法（如快速排序、堆排序和归并排序）。


def insertion_sort(arr):
    for i in range(1, len(arr)):
        key = arr[i]
        j = i-1
        while j >=0 and key < arr[j]:  # 将元素向右移动，为插入操作腾出空间
            arr[j+1] = arr[j]
            j -= 1
        arr[j+1] = key

# 测试插入排序
arr = [12, 11, 13, 5, 6, 7]
insertion_sort(arr)
print("Sorted array is:", arr)

from typing import List
from collections import heapq

# 离线查询 + 最小堆
class Solution:
    def leftmostBuildingQueries(self, heights: List[int], queries: List[List[int]]) -> List[int]:
        ans = [-1] * len(queries)
        left = [[] for _ in heights]
        for qi, (i, j) in enumerate(queries):
            if i > j:
                i, j = j, i  # 保证 i <= j
            if i == j or heights[i] < heights[j]:
                ans[qi] = j  # i 直接跳到 j
            else:
                left[j].append((heights[i], qi))  # 离线

        h = []
        for i, x in enumerate(heights):  # 从小到大枚举下标 i
            while h and h[0][0] < x:
                ans[heapq.heappop(h)[1]] = i  # 可以跳到 i（此时 i 是最小的）
            for p in left[i]:
                heapq.heappush(h, p)  # 后面再回答
        return ans

# 可读性优化
from typing import List
from collections import heapq

class Solution:
    def leftmostBuildingQueries(self, heights: List[int], queries: List[List[int]]) -> List[int]:
        # 初始化答案数组，初始值设为 -1
        answers = [-1] * len(queries)

        # 'left_jump' 用于存储每个建筑左侧的跳跃请求
        left_jump = [[] for _ in heights]

        # 预处理查询，调整顺序并处理直接跳跃的情况
        for query_index, (left, right) in enumerate(queries):
            if left > right:
                left, right = right, left  # 保证 start <= end
            if left == right or heights[left] < heights[right]:
                answers[query_index] = right  # 如果 start 和 end 相等或 start 建筑更矮，直接跳到 end
            else:
                left_jump[right].append((heights[left], query_index))  # 否则，记录需要从 start 跳跃的请求

        # 使用最小堆处理跳跃请求
        min_heap = []
        for index, height in enumerate(heights):
            # 移除所有当前建筑比较高的堆元素
            while min_heap and min_heap[0][0] < height:
                _, query_index = heapq.heappop(min_heap)
                answers[query_index] = index  # 当前建筑是最近的更高建筑

            # 将当前建筑左侧的跳跃请求加入堆中
            for jump in left_jump[index]:
                heapq.heappush(min_heap, jump)

        return answers

                

        
# 输入：heights = [6,4,8,5,2,7], queries = [[0,1],[0,3],[2,4],[3,4],[2,2]]
# 输出：[2,5,-1,5,2]        