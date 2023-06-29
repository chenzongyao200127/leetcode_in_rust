# 60_Permutation_Sequence
# https://leetcode.cn/problems/permutation-sequence/


# 给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。
# 按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

# "123"
# "132"
# "213"
# "231"
# "312"
# "321"
# 给定 n 和 k，返回第 k 个排列。


class Solution:
    def getPermutation(self, n: int, k: int) -> str:
        def find_decreasing_suffix_start(nums) -> int:
            for i in range(len(nums) - 1, 0, -1):
                if nums[i - 1] < nums[i]:
                    return i
            return 0

        def find_swap_index(nums, start: int) -> int:
            for i in range(len(nums) - 1, start - 1, -1):
                if nums[i] > nums[start - 1]:
                    return i

        def nextPermutation(nums):
            suffix_start = find_decreasing_suffix_start(nums)

            if suffix_start == 0:
                nums.sort()
                return

            swap_index = find_swap_index(nums, suffix_start)
            nums[suffix_start - 1], nums[swap_index] = nums[swap_index], nums[suffix_start - 1]
            nums[suffix_start:] = sorted(nums[suffix_start:])

        nums = [i for i in range(1, n + 1)]

        for _ in range(1, k):
            nextPermutation(nums)

        return ''.join(str(n) for n in nums)
    
# 一句话题解：以下给出了两种方法，思路其实是一样的：通过计算剩余数字个数的阶乘数，一位一位选出第 k 个排列的数位。
# 思路分析：容易想到，使用同「力扣」第 46 题： 全排列 的回溯搜索算法，依次得到全排列，输出第 k 个全排列即可。事实上，我们不必求出所有的全排列。
# 基于以下几点考虑：
# 所求排列 一定在叶子结点处得到，进入每一个分支，可以根据已经选定的数的个数，进而计算还未选定的数的个数，然后计算阶乘，就知道这一个分支的 叶子结点 的个数：
# 如果 k 大于这一个分支将要产生的叶子结点数，直接跳过这个分支，这个操作叫「剪枝」；
# 如果 k 小于等于这一个分支将要产生的叶子结点数，那说明所求的全排列一定在这一个分支将要产生的叶子结点里，需要递归求解。

class Solution:
    def getPermutation(self, n: int, k: int) -> str:
        # Define a recursive helper function to perform a depth-first search (DFS)
        def dfs(n, k, index, path):
            # Base case: when the index reaches n, the path is complete, so return
            if index == n:
                return

            # Calculate the number of remaining permutations using the factorial array
            cnt = factorial[n - 1 - index]

            # Loop through all possible candidates from 1 to n
            for i in range(1, n + 1):
                # Skip the number if it has already been used in the current path
                if used[i]:
                    continue

                # If the cnt is less than k, decrement k by cnt and move to the next candidate
                if cnt < k:
                    k -= cnt
                    continue

                # If the cnt is greater or equal to k, add the candidate to the path,
                # mark it as used, and perform the DFS for the next index
                path.append(i)
                used[i] = True
                dfs(n, k, index + 1, path)

                # Return early, as there is no need to explore further candidates
                return

        # Return an empty string if n is 0
        if n == 0:
            return ""

        # Initialize an array to track which numbers have been used
        used = [False for _ in range(n + 1)]

        # Initialize an array to store the path, which will be the result
        path = []

        # Create a factorial array to store the factorials from 1 to n
        factorial = [1 for _ in range(n + 1)]
        for i in range(2, n + 1):
            factorial[i] = factorial[i - 1] * i

        # Start the DFS with the given n, k, and an initial index of 0
        dfs(n, k, 0, path)

        # Convert the path to a string and return it
        return ''.join([str(num) for num in path])