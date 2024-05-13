# 二分查找算法是一种在有序数组中查找某个特定元素的搜索算法。它的工作原理是反复将待搜索的部分数组划分为两半。如果目标值与中间元素相等，则查找成功。如果目标值小于中间元素，则在左半部分数组中继续查找；反之，如果目标值大于中间元素，则在右半部分数组中继续查找。

# 二分查找算法的主要优点是查找速度快，时间复杂度为O(log n)，其中n为数组的元素数量。但它的前提是数组必须是有序的。

# ### 算法步骤：
# 1. 定义三个指针，low, mid 和 high，分别指向数组的开始、中间和结束位置。
# 2. 检查mid位置的元素是否是要查找的目标元素。
# 3. 如果是，返回mid的位置。
# 4. 如果目标元素大于mid位置的元素，将low设置为mid+1，并回到第2步。
# 5. 如果目标元素小于mid位置的元素，将high设置为mid-1，并回到第2步。
# 6. 如果low>high，则目标元素不在数组中，返回-1或其他特定的错误标识。

# ### Python代码示例：

# ```python

def binary_search(arr, x):
    """
    使用二分查找算法在有序数组中查找目标元素的索引。

    参数:
    arr -- 有序数组。
    x -- 目标元素。

    返回:
    index -- 目标元素的索引，如果未找到则返回-1。
    """
    low, high = 0, len(arr) - 1

    while low <= high:
        mid = (low + high) // 2
        mid_value = arr[mid]

        if mid_value == x:
            return mid
        elif mid_value < x:
            low = mid + 1
        else:
            high = mid - 1

    return -1

# ```

# ### 注意事项：
# - 二分查找仅适用于有序数组。如果数组未排序，则需要先排序，或者使用其他搜索方法。
# - 对于非唯一值，二分查找可能无法找到所有匹配的索引，因为它会在找到第一个匹配的索引后停止。
# - 虽然二分查找是高效的，但在极小的数组中，线性查找可能会更快。


# 增加测试用例
# 测试用例1
arr = [2, 3, 4, 10, 40]
x = 10
assert binary_search(arr, x) == 3


# 桶排序（Bucket Sort）是一种分布式排序算法，它将元素分散到多个称为“桶”的区间中，
# 然后每个桶内部再进行独立排序，最后将各个桶中的元素按顺序合并，完成排序过程。
# 桶排序特别适用于输入数据均匀分布在一个范围内的情况。

# 下面是一个用 Python 实现的桶排序的简单模板：
def bucket_sort(arr, bucket_size=5):
    """
    桶排序函数

    参数:
    arr -- 待排序数组
    bucket_size -- 桶的大小，默认为5
    """
    if len(arr) == 0:
        return arr

    # 确定最小和最大值
    min_value = min(arr)
    max_value = max(arr)

    # 初始化桶
    bucket_count = (max_value - min_value) // bucket_size + 1
    buckets = [[] for _ in range(bucket_count)]

    # 分配元素到各个桶
    for i in range(len(arr)):
        index = (arr[i] - min_value) // bucket_size
        buckets[index].append(arr[i])

    # 对每个桶进行排序
    arr.clear()
    for i in range(len(buckets)):
        buckets[i] = sorted(buckets[i])
        arr.extend(buckets[i])

    return arr


# 示例使用
if __name__ == "__main__":
    array = [29, 25, 3, 49, 9, 37, 21, 43]
    sorted_array = bucket_sort(array)
    print("Sorted Array:", sorted_array)


# # 代码解释
# 1. ** 参数 **：`bucket_size` 参数用于定义每个桶可以包含的元素范围大小。
# 2. ** 初始化桶 **：根据最小值、最大值和桶的大小计算出需要多少个桶，并初始化。
# 3. ** 分配元素 **：遍历原始数组，根据元素值计算它应该放入哪个桶。
# 4. ** 排序和合并 **：对每个桶内的元素进行排序（可以使用任何排序算法，这里使用了 Python 的内置排序），然后将排序后的元素合并回原数组。

# # 注意点
# - 桶排序的效率依赖于输入数据的分布，如果数据分布非常不均匀，则桶排序的效率并不高。
# - 桶的大小和数量的选择对性能有显著影响。
# - 本例中使用的是线性时间的排序算法作为内部排序，对每个桶内部的排序可以选择合适的算法。

# 这个桶排序模板简单且直观，适用于教学和处理均匀分布数据的场景。对于实际应用，可能需要根据数据特性调整桶的大小和排序算法。
