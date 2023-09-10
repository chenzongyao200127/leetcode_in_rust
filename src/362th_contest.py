def covered_points(nums):
    
    # 将给定的区间按照起始点排序
    nums.sort(key=lambda x: x[0])
    
    merged = []  # 用于存储合并后的区间
    
    # 遍历排序后的每个区间
    for start, end in nums:
        # 如果merged为空或merged中的最后一个区间的结束点小于当前区间的起始点
        # 这意味着它们没有重叠，所以我们可以直接将当前区间添加到merged中
        if not merged or merged[-1][1] < start:
            merged.append([start, end])
        else:
            # 否则，如果它们有重叠，我们更新merged中最后一个区间的结束点
            # 为两个区间中的最大结束点
            merged[-1][1] = max(merged[-1][1], end)
    
    count = 0  # 用于计算覆盖的点数
    
    # 遍历合并后的每个区间
    for start, end in merged:
        # 对于每个区间，其覆盖的点数为结束点减起始点再加1
        # 因为这是一个闭区间
        count += end - start + 1
    
    # 返回总的覆盖点数
    return count






class Solution:
    def isReachableAtTime(self, sx: int, sy: int, fx: int, fy: int, t: int) -> bool:
        fx = fx + (0 - sx)
        fy = fy + (0 - sy)
        fx = abs(fx)
        fy = abs(fy)
        # print((fx, fy))
        tmp = max(fx, fy)
        if tmp == 0:
            return t != 1
        return tmp <= t   
        


s = Solution()
ans = s.isReachableAtTime(sx = 2, sy = 4, fx = 7, fy = 7, t = 6)
print(ans)
        
s = Solution()
ans = s.isReachableAtTime(sx = 3, sy = 1, fx = 7, fy = 3, t = 3)
print(ans)

s = Solution()
ans = s.isReachableAtTime(sx = 1, sy = 2, fx = 1, fy = 2, t = 1)
print(ans) # false

s = Solution()
ans = s.isReachableAtTime(sx = 1, sy = 2, fx = 1, fy = 2, t = 3)
print(ans) # true



from typing import List
from collections import deque

class Solution:
    def minimumMoves(self, grid: List[List[int]]) -> int:
        N = 3  # 网格的大小
        
        # 定义目标状态：即每个位置恰好有一个石头的状态
        target = tuple(sorted((i, j) for i in range(N) for j in range(N)))
        stones = []
        
        # 遍历网格，找到所有石头的位置
        for i in range(N):
            for j in range(N):
                # 对于多个石头在同一位置的情况，记录所有石头的位置
                for k in range(grid[i][j]):
                    stones.append((i, j))
                    
        # 初始化石头的起始状态
        start = tuple(stones)

        # 使用BFS搜索，queue中存储当前的状态和到达该状态所需的步数
        queue = deque([(start, 0)])
        
        # 使用集合记录已经访问过的状态，避免重复搜索
        visited = set([start])
        
        # 当队列非空时，继续搜索
        while queue:
            # 从队列头部取出一个状态
            state, steps = queue.popleft()
            # 如果该状态为目标状态，返回所需步数
            if state == target:
                return steps
            
            # 遍历当前状态中的每一个石头
            for i, (x, y) in enumerate(state):
                # 尝试四个方向的移动
                for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    # 计算石头移动后的新位置
                    nx, ny = x + dx, y + dy
                    # 检查新位置是否在网格内，且该位置没有其他石头
                    if 0 <= nx < N and 0 <= ny < N and (nx, ny) not in state:
                        # 创建新的状态，并更新移动后的石头位置
                        newState = list(state)
                        newState[i] = (nx, ny)
                        # 对新状态的石头位置进行排序，确保状态的唯一性
                        newState = tuple(sorted(newState))
                        # 如果新状态没有被访问过，将其加入队列和访问集合
                        if newState not in visited:
                            visited.add(newState)
                            queue.append((newState, steps + 1))
                            
        # 如果没有找到解，返回-1
        return -1



s = Solution()
ans = s.minimumMoves(grid = [[1,1,0],[1,1,1],[1,2,1]])
print(ans)  # 3

s = Solution()
ans = s.minimumMoves([[1,3,0],[1,0,0],[1,0,3]])
print(ans)  # 4     

s = Solution()
ans = s.minimumMoves([[0,7,1],[0,1,0],[0,0,0]])
print(ans)  # 4    

