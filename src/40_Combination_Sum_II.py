# 40_Combination_Sum_II
# https://leetcode.cn/problems/combination-sum-ii/

# 给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
# candidates 中的每个数字在每个组合中只能使用 一次 。
# 注意：解集不能包含重复的组合。 
class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        cur_com = []
        ans = []
        def dfs(candidates, target, cur_com, start_idx, cur_sum):
            if cur_sum > target or start_idx > len(candidates):
                return
            
            if cur_sum == target:
                ans.append(cur_com[:])
                return
            
            for i in range(start_idx, len(candidates)):
                if i > start_idx and candidates[i] == candidates[i - 1]:
                    continue
                
                cur_com.append(candidates[i])
                dfs(candidates, target, cur_com, i + 1, cur_sum + candidates[i])
                cur_com.pop()
                
        dfs(candidates, target, cur_com, 0, 0)
        return ans

# print(combinationSum2([2,5,2,1,2],5))



class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        def dfs(pos: int, rest: int):
            nonlocal sequence
            if rest == 0:
                ans.append(sequence[:])
                return
            if pos == len(freq) or rest < freq[pos][0]:
                return
            
            dfs(pos + 1, rest)

            most = min(rest // freq[pos][0], freq[pos][1])
            for i in range(1, most + 1):
                sequence.append(freq[pos][0])
                dfs(pos + 1, rest - i * freq[pos][0])
            sequence = sequence[:-most]
        
        freq = sorted(collections.Counter(candidates).items())
        ans = list()
        sequence = list()
        dfs(0, target)
        return ans



class Solution:
    def __init__(self):
        self.paths = []
        self.path = []

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        '''
        类似于求三数之和，求四数之和，为了避免重复组合，需要提前进行数组排序
        '''
        self.paths.clear()
        self.path.clear()
        # 必须提前进行数组排序，避免重复
        candidates.sort()
        self.backtracking(candidates, target, 0, 0)
        return self.paths

    def backtracking(self, candidates: List[int], target: int, sum_: int, start_index: int) -> None:
        # Base Case
        if sum_ == target:
            self.paths.append(self.path[:])
            return
        
        # 单层递归逻辑
        for i in range(start_index, len(candidates)):
            # 剪枝，同39.组合总和
            if sum_ + candidates[i] > target:
                return
            
            # 跳过同一树层使用过的元素
            if i > start_index and candidates[i] == candidates[i-1]:
                continue
            
            sum_ += candidates[i]
            self.path.append(candidates[i])
            self.backtracking(candidates, target, sum_, i+1)
            self.path.pop()             # 回溯，为了下一轮for loop
            sum_ -= candidates[i]       # 回溯，为了下一轮for loop
        
        