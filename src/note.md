# Heap

堆（heap），在计算机科学中，特别是在最小堆和最大堆的形式下，是一种常见的数据结构。
堆通常是通过数组来实现的，并且具有一些特殊的属性使其成为有效的优先队列。以下是堆的基本概念和底层实现：

### 基本概念

1. **完全二叉树**：堆通常是一个完全二叉树，这意味着所有的层都是满的，除了最后一层可能不满，但最后一层的元素从左到右填充。
2. **堆属性**：
   - **最大堆**：每个父节点的值大于或等于其子节点的值。
   - **最小堆**：每个父节点的值小于或等于其子节点的值。

### 底层实现

1. **数组表示**：
   - 在堆的数组表示中，根节点位于数组的开始（索引 0 或 1，取决于实现）。
   - 对于数组中任意一个位置 `i` 的元素：
     - 它的左子节点位于 `2*i + 1`（如果堆从索引 0 开始）。
     - 它的右子节点位于 `2*i + 2`（如果堆从索引 0 开始）。
     - 它的父节点位于 `(i-1)/2`（如果堆从索引 0 开始）。

2. **基本操作**：
   - **插入**（Heapify Up）：
     - 新元素被添加到数组的末尾。
     - 为了维护堆的属性，执行一个向上调整（`heapify up`）的过程：新元素与其父节点比较，如果违反了堆的属性（在最小堆中父节点比新元素大，在最大堆中父节点比新元素小），则与父节点交换。这个过程一直持续到根节点或者没有违反堆属性为止。
   - **删除**（Heapify Down）：
     - 在最小堆中删除根节点（最小元素），在最大堆中删除根节点（最大元素）。
     - 通常将数组最后一个元素移动到根位置。
     - 执行向下调整（`heapify down`）的过程：父节点与其子节点比较，如果违反了堆的属性，则与最小（在最小堆中）或最大（在最大堆中）的子节点交换。这个过程持续直到叶节点或者没有违反堆属性为止。

3. **时间复杂度**：
   - 插入和删除操作通常具有 O(log n) 的时间复杂度，其中 n 是堆中的元素数量。
   - 查找最小元素（最小堆）或最大元素（最大堆）的操作是 O(1)，因为这些元素总是位于根节点。

堆的这种实现方式，特别是其在数组中的紧凑布局，使其成为优先队列和其他相关应用（如堆排序）的高效选择。

假设我们有一个具有以下元素的最小堆：`10, 20, 30, 40, 50, 60, 70`。在一个二叉堆中，这些元素将按照特定的顺序存储在数组中。

在最小堆中，每个节点的值都小于或等于其子节点的值。以下是这个堆的一个可能的布局：

```
          10
         /  \
       20    30
      / \    / \
     40  50 60  70
```

以数组形式存储时，这个堆看起来是这样的：`[10, 20, 30, 40, 50, 60, 70]`。数组索引与堆中的位置之间的关系如下：

- 索引 0：根节点（10）
- 索引 1：20（10 的左子节点）
- 索引 2：30（10 的右子节点）
- 索引 3：40（20 的左子节点）
- 索引 4：50（20 的右子节点）
- 索引 5：60（30 的左子节点）
- 索引 6：70（30 的右子节点）

这种布局利用了完全二叉树的性质，使得可以通过数组索引快速地访问任何节点的父节点和子节点。例如，给定一个索引 `i`，则：

- 左子节点的索引是 `2*i + 1`
- 右子节点的索引是 `2*i + 2`
- 父节点的索引是 `(i-1)/2`（当 `i` 不是根节点时）

> 请注意，这只是一个示例布局，实际的堆可能会有不同的形状，具体取决于元素的插入和移除顺序。

在 Python 中实现一个最小堆可以通过使用列表来手动管理堆的结构，也可以利用 Python 标准库中的 `heapq` 模块，后者提供了堆操作的函数。
下面我将展示如何用 Python 手动实现一个最小堆的类，包括插入和删除最小元素的方法。

### 手动实现最小堆

```python
class MinHeap:
    def __init__(self):
        self.heap = []

    def parent(self, i):
        return (i - 1) // 2

    def left_child(self, i):
        return 2 * i + 1

    def right_child(self, i):
        return 2 * i + 2

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
        while i != 0 and self.heap[self.parent(i)] > self.heap[i]:
            self.swap(i, self.parent(i))
            i = self.parent(i)

    def remove_min(self):
        if len(self.heap) == 0:
            raise IndexError("Heap is empty")
        min_elem = self.heap[0]
        self.heap[0] = self.heap[-1]
        self.heap.pop()
        self.heapify_down(0)
        return min_elem

    def heapify_down(self, i):
        while self.has_left_child(i):
            smaller_child = self.left_child(i)
            if self.has_right_child(i) and self.heap[self.right_child(i)] < self.heap[smaller_child]:
                smaller_child = self.right_child(i)

            if self.heap[i] <= self.heap[smaller_child]:
                break

            self.swap(i, smaller_child)
            i = smaller_child
```

### 使用示例

```python
heap = MinHeap()
heap.insert(3)
heap.insert(1)
heap.insert(6)
heap.insert(5)
heap.insert(2)
heap.insert(4)

print(heap.remove_min())  # 输出: 1
print(heap.remove_min())  # 输出: 2
```

这个 `MinHeap` 类通过一个列表来存储堆，并提供了插入新元素和删除最小元素的方法。
`heapify_up` 和 `heapify_down` 方法用于在插入和删除操作后保持堆的属性。

> 注意：这只是一个简单的示例实现。在实际应用中，通常会使用 Python 的 `heapq` 模块，因为它是高度优化的。
