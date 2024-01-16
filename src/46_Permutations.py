# 46_Permutations
# https://leetcode.cn/problems/permutations/

# 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。
class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        return list(itertools.permutations(nums))


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)

        def dfs(com, nums, n):
            if len(com) == n:
                ans.append(com[:])

            for i in range(len(nums)):
                com.append(nums[i])
                new_num = nums[:i] + nums[i+1:]
                dfs(com, new_num, n)
                com.pop()

        dfs([], nums, n)
        return ans


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        result = [[]]
        i = 0  # 记录要被插入的数据在nums中的位置
        while True:
            if len(result[0]) == len(nums):  # 如果result[0]长度和nums一致，则说明所有组合已列举完毕，直接返回结果
                return result
            cur = result.pop(0)  # 将result中首元素拿出来进行插入
            i = len(cur)  # 已经有len(cur)个元素被插入，所以当前要插入的元素为nums[i]
            for x in range(len(cur) + 1):  # 每个元素有len(cur)个位置可供插入
                mid = cur.copy()  # 每次插入不改变cur的值
                mid.insert(x, nums[i])
                result.append(mid.copy())


class Solution:
    def permute(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """
        def backtrack(first=0):
            # 所有数都填完了
            if first == n:
                res.append(nums[:])

            for i in range(first, n):
                # 动态维护数组
                nums[first], nums[i] = nums[i], nums[first]
                # 继续递归填下一个数
                backtrack(first + 1)
                # 撤销操作
                nums[first], nums[i] = nums[i], nums[first]

        n = len(nums)
        res = []
        backtrack()
        return res
