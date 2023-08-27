# 示例 1：
# 输入：moves = "L_RL__R"
# 输出：3
# 解释：可以到达的距离原点 0 最远的点是 -3 ，移动的序列为 "LLRLLLR" 。

# 示例 2：
# 输入：moves = "_R__LL_"
# 输出：5
# 解释：可以到达的距离原点 0 最远的点是 -5 ，移动的序列为 "LRLLLLL" 。

# 示例 3：
# 输入：moves = "_______"
# 输出：7
# 解释：可以到达的距离原点 0 最远的点是 7 ，移动的序列为 "RRRRRRR" 。


class Solution:
    def furthestDistanceFromOrigin(self, moves: str) -> int:
        cnt = 0 
        x = 0
        for s in moves:
            if s == "R":
                cnt += 1
            elif s == "L":
                cnt -= 1
            else:
                x += 1
        
        return abs(cnt) + x
    

        
def minSum(n, target):
    nums = []
    available = [True] * (target + n)

    i = 1
    while len(nums) < n:
        if available[i] and (target - i > i) and available[target - i]:
            nums.append(i)
            available[i] = False
            available[target - i] = False
        elif available[i] and (target - i <= i): 
            nums.append(i)
            available[i] = False
        i += 1

    return sum(nums)



# 给你一个下标从 0 开始的数组 nums ，它包含 非负 整数，且全部为 2 的幂，同时给你一个整数 target 。

# 一次操作中，你必须对数组做以下修改：

# 选择数组中一个元素 nums[i] ，满足 nums[i] > 1 。
# 将 nums[i] 从数组中删除。
# 在 nums 的 末尾 添加 两个 数，值都为 nums[i] / 2 。
# 你的目标是让 nums 的一个 子序列 的元素和等于 target ，请你返回达成这一目标的 最少操作次数 。如果无法得到这样的子序列，请你返回 -1 。

# 数组中一个 子序列 是通过删除原数组中一些元素，并且不改变剩余元素顺序得到的剩余数组。

# 示例 1：
# 输入：nums = [1,2,8], target = 7
# 输出：1
# 解释：第一次操作中，我们选择元素 nums[2] 。数组变为 nums = [1,2,4,4] 。
# 这时候，nums 包含子序列 [1,2,4] ，和为 7 。
# 无法通过更少的操作得到和为 7 的子序列。

# 示例 2：
# 输入：nums = [1,32,1,2], target = 12
# 输出：2
# 解释：第一次操作中，我们选择元素 nums[1] 。数组变为 nums = [1,1,2,16,16] 。
# 第二次操作中，我们选择元素 nums[3] 。数组变为 nums = [1,1,2,16,8,8] 。
# 这时候，nums 包含子序列 [1,1,2,8] ，和为 12 。
# 无法通过更少的操作得到和为 12 的子序列。

# 示例 3：
# 输入：nums = [1,32,1], target = 35
# 输出：-1
# 解释：无法得到和为 35 的子序列。


# 此函数的目的是找到一个最小操作数，以便让 `nums`（一个所有元素都是2的幂的整数列表）的子序列之和等于目标值 `target`。
# 函数的主要思路和关键步骤如下：
# 1. **基础检查**：如果 `nums` 中所有数字的总和小于 `target`，那么无法达到目标，因此返回 -1。
# 2. **计数**：使用 `Counter` 类统计 `nums` 中每个数字的出现次数。
# 3. **初始化**：初始化两个跟踪器：一个用于操作数 (`operations`)，另一个用于当前总和 (`total_sum`)。
# 4. **检查2的每个幂**：由于所有数字都是2的幂，所以我们检查从2^0到2^30的每一个幂。
#     - 我们首先更新当前的总和 (`total_sum`)，将其增加为当前2的幂 (`1 << i`) 的出现次数乘以当前的2的幂。
#     - 然后，检查目标 `target` 的第 `i` 位是否设置（即是否为1）。如果未设置，我们继续下一个幂。
#     - 从 `total_sum` 中减去当前的2的幂。如果结果仍然为正，我们继续。
#     - 否则，这意味着我们需要找到一个更大的2的幂并将其“拆分”成较小的幂，以满足目标。为此，我们从 `i+1` 开始，
#       查找下一个存在的2的幂，并逐渐“拆分”它，同时增加操作数。
# 5. **返回结果**：函数最后返回 `operations`，它表示为使子序列之和等于目标所需的最小操作数。

# 需要注意的是，函数中的某些细节（例如为什么选择2^30作为最大的幂）可能基于特定的应用场景或上下文，不一定在所有情况下都适用。

from collections import Counter
from typing import List

def min_operations(nums: List[int], target: int) -> int:
    # If the sum of all numbers in nums is less than target, it's not possible to achieve the target.
    if sum(nums) < target:
        return -1

    # Count the occurrence of each number in nums.
    count = Counter(nums)

    # Initialize operations counter and sum tracker.
    operations = total_sum = 0

    # Iterate through powers of 2 (as given numbers are powers of 2).
    for i in range(31):  # Considering 2^30 as the maximum power of 2, as numbers are non-negative.
        
        # Update the current total sum with the count of current power of 2.
        total_sum += count[1 << i] << i
        
        # Check if the ith bit in target is set.
        if (target >> i & 1) == 0:
            continue
        
        # Deduct the current power of 2 from the total sum.
        total_sum -= 1 << i
        
        # If the current total sum is non-negative, continue.
        if total_sum >= 0:
            continue
        
        # Otherwise, search for a larger power of 2 in nums to split.
        for j in range(i + 1, 31):
            if count[1 << j]:
                # Increment operations by the difference in powers.
                operations += j - i
                
                # Deduct the count of the found power of 2 and update the total sum.
                count[1 << j] -= 1
                total_sum += 1 << j
                
                # We found a number to split, so break out of the loop.
                break

    return operations


# 示例
nums = [1, 2, 8]
target = 7
print(min_operations(nums, target))  # 1

nums = [1,32,1,2]
target = 12
print(min_operations(nums, target))  # 2

nums = [1,2,8,32,16]
target = 20
print(min_operations(nums, target))  # 1

nums = [1,32,1]
target = 35
print(min_operations(nums, target))  # -1

nums = [16,128,32]
target = 1
print(min_operations(nums, target))  # 4

nums = [16,16,4]
target = 3
print(min_operations(nums, target))  # 2



# 给你一个长度为 n 下标从 0 开始的整数数组 receiver 和一个整数 k 。
# 总共有 n 名玩家，玩家 编号 互不相同，且为 [0, n - 1] 中的整数。
# 这些玩家玩一个传球游戏，receiver[i] 表示编号为 i 的玩家会传球给编号为 receiver[i] 的玩家。
# 玩家可以传球给自己，也就是说 receiver[i] 可能等于 i 。

# 你需要从 n 名玩家中选择一名玩家作为游戏开始时唯一手中有球的玩家，球会被传 恰好 k 次。
# 如果选择编号为 x 的玩家作为开始玩家，定义函数 f(x) 表示从编号为 x 的玩家开始，
# k 次传球内所有接触过球玩家的编号之 和 ，如果有玩家多次触球，则 累加多次 。
# 换句话说， f(x) = x + receiver[x] + receiver[receiver[x]] + ... + receiver(k)[x] 。

# 你的任务时选择开始玩家 x ，目的是 最大化 f(x) 。
# 请你返回函数的 最大值 。
# 注意：receiver 可能含有重复元素。

