# 332_Reconstruct_Itinerary
# https://leetcode.cn/problems/reconstruct-itinerary/

# 给你一份航线列表 tickets ，其中 tickets[i] = [fromi, toi] 表示飞机出发和降落的机场地点。请你对该行程进行重新规划排序。

# 所有这些机票都属于一个从 JFK（肯尼迪国际机场）出发的先生，所以该行程必须从 JFK 开始。如果存在多种有效的行程，请你按字典排序返回最小的行程组合。

# 例如，行程 ["JFK", "LGA"] 与 ["JFK", "LGB"] 相比就更小，排序更靠前。
# 假定所有机票至少存在一种合理的行程。且所有的机票 必须都用一次 且 只能用一次。

from typing import List

# 超时
def findItinerary(tickets):
    airports = []
    for ticket in tickets:
        if ticket[0] not in airports:
            airports.append(ticket[0])
        if ticket[1] not in airports:
            airports.append(ticket[1])
    
    def findAirpportIdx(airport, airports):
        for i in range(len(airports)):
            if airports[i] == airport:
                return i
    
    n = len(airports)
    map = [[0] * n for _ in range(n)]
    for ticket in tickets:
        source = findAirpportIdx(ticket[0], airports)
        destation = findAirpportIdx(ticket[1], airports)
        map[source][destation] += 1
    
    start = findAirpportIdx("JFK", airports)
    path = ["JFK"]
    routes = []
    
    def dfs(map, routes, start, path):
        if all(0 == row[i] for row in map for i in range(len(row))):
            routes.append(path[:])
        
        for j in range(len(map[start])):
            if map[start][j] != 0:
                path.append(airports[j])
                next_start = j
                map[start][j] -= 1
                dfs(map, routes, next_start, path)
                map[start][j] += 1
                path.pop()
    
    dfs(map, routes, start, path)
    routes.sort()
    print(routes)
    return routes[0]
            
print(findItinerary([["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]))


# 我们化简本题题意：给定一个 n 个点 m 条边的图，要求从指定的顶点出发，经过所有的边恰好一次（可以理解为给定起点的「一笔画」问题），使得路径的字典序最小。

# 这种「一笔画」问题与欧拉图或者半欧拉图有着紧密的联系，下面给出定义：

# 通过图中所有边恰好一次且行遍所有顶点的通路称为欧拉通路；
# 通过图中所有边恰好一次且行遍所有顶点的回路称为欧拉回路；
# 具有欧拉回路的无向图称为欧拉图；
# 具有欧拉通路但不具有欧拉回路的无向图称为半欧拉图。
# 因为本题保证至少存在一种合理的路径，也就告诉了我们，这张图是一个欧拉图或者半欧拉图。我们只需要输出这条欧拉通路的路径即可。

# 如果没有保证至少存在一种合理的路径，我们需要判别这张图是否是欧拉图或者半欧拉图，具体地：

# 对于无向图 G，G 是欧拉图当且仅当 G 是连通的且没有奇度顶点。
# 对于无向图 G，G 是半欧拉图当且仅当 G 是连通的且 G 中恰有 0 个或 2个奇度顶点。
# 对于有向图 G，G 是欧拉图当且仅当 G 的所有顶点属于同一个强连通分量且每个顶点的入度和出度相同。
# 对于有向图 G，G 是半欧拉图当且仅当如果将 G 中的所有有向边退化为无向边时，那么 G 的所有顶点属于同一个强连通分量；
# 最多只有一个顶点的出度与入度差为 1；最多只有一个顶点的入度与出度差为 1;所有其他顶点的入度和出度相同。


from collections import defaultdict

# 为了提高代码性能并避免超时，您可以针对以下几点进行优化：

# 使用字典而非列表来存储机场索引，以加速查找。
# 使用字典而非二维数组来存储航班地图。
# 使用深度优先搜索 (DFS) 搜索路径，但在搜索过程中进行字典排序，这样就不需要在最后对整个 routes 进行排序。

# 由于题目中说必然存在一条有效路径(至少是半欧拉图)，所以算法不需要回溯（既加入到结果集里的元素不需要删除）
# 整个图最多存在一个死胡同(出度和入度相差1），且这个死胡同一定是最后一个访问到的，否则无法完成一笔画。
# DFS的调用其实是一个拆边的过程（既每次递归调用删除一条边，所有子递归都返回后，再将当前节点加入结果集保证了结果集的逆序输出），
# 一定是递归到这个死胡同（没有子递归可以调用）后递归函数开始返回。所以死胡同是第一个加入结果集的元素。
# 最后逆序的输出即可。

# 方法一：
# Hierholzer 算法

# 思路及算法
# Hierholzer 算法用于在连通图中寻找欧拉路径，其流程如下：
#     从起点出发，进行深度优先搜索。
#     每次沿着某条边从某个顶点移动到另外一个顶点的时候，都需要删除这条边。
#     如果没有可移动的路径，则将所在节点加入到栈中，并返回。

#     当我们顺序地考虑该问题时，我们也许很难解决该问题，因为我们无法判断当前节点的哪一个分支是「死胡同」分支。
#     不妨倒过来思考。我们注意到只有那个入度与出度差为 1 的节点会导致死胡同。而该节点必然是最后一个遍历到的节点。
# 我们可以改变入栈的规则，当我们遍历完一个节点所连的所有节点后，我们才将该节点入栈（即逆序入栈）。

#     对于当前节点而言，从它的每一个非「死胡同」分支出发进行深度优先搜索，都将会搜回到当前节点。
# 而从它的「死胡同」分支出发进行深度优先搜索将不会搜回到当前节点。也就是说当前节点的死胡同分支将会优先于其他非「死胡同」分支入栈。
#     这样就能保证我们可以「一笔画」地走完所有边，最终的栈中逆序地保存了「一笔画」的结果。我们只要将栈中的内容反转，即可得到答案。


class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        # 创建一个集合用于存储所有机场名，去重并保留唯一机场名
        airports = set()
        for ticket in tickets:
            airports.add(ticket[0])
            airports.add(ticket[1])

        # 创建一个嵌套的 defaultdict，用于存储从一个城市到另一个城市的航班数量
        flight_map = defaultdict(lambda: defaultdict(int))

        # 遍历所有机票，统计每个城市之间的航班数量
        for ticket in tickets:
            source, destation = ticket
            flight_map[source][destation] += 1

        # 从 JFK 机场开始寻找行程
        start = "JFK"
        # 初始化 route 用于存储最终的行程
        route = []

        # 定义一个递归的深度优先搜索 (DFS) 函数，用于找到合适的行程
        def dfs(airport):
            # 当当前机场还有航班时
            while flight_map[airport]:
                # 找到所有可选目标机场中字典序最小的一个
                next_airport = min(flight_map[airport].keys())
                # 将该航班计数减一
                flight_map[airport][next_airport] -= 1
                # 如果该航班数量已经减为 0，从字典中删除该航班
                if flight_map[airport][next_airport] == 0:
                    del flight_map[airport][next_airport]
                # 对下一个机场进行深度优先搜索
                dfs(next_airport)
            # 当前机场没有可选航班时，将机场添加到 route 中
            route.append(airport)

        # 从 start 机场开始进行深度优先搜索
        dfs(start)
        # 因为 DFS 最终得到的行程是反向的，所以需要翻转 route 列表
        return route[::-1]
    

# 这行代码创建了一个嵌套的字典，其中外层字典和内层字典都是 defaultdict 类型。

# defaultdict 类型是 Python collections 模块提供的一个特殊字典类型。
# 当您尝试访问一个尚不存在的键时，defaultdict 会自动创建该键并将其值设置为一个默认值。默认值是通过提供一个可调用对象（如工厂函数）在创建字典时定义的。

# 在这个例子中，外层字典的默认值是另一个 defaultdict，这意味着当您尝试访问一个尚不存在的键时，将自动创建一个新的 defaultdict。
# 内层字典的默认值是 int 类型，这意味着当您尝试访问一个尚不存在的键时，将自动创建一个值为 0 的整数。

# 让我们详细分析这行代码：

# defaultdict(int)：创建一个新的 defaultdict，其默认值为整数。当您访问该字典中不存在的键时，它将自动创建一个值为 0 的整数。

# lambda: defaultdict(int)：定义一个匿名函数（lambda 函数），这个函数不接受任何参数并返回一个新的 defaultdict(int)。
# 这个匿名函数将作为外层字典的默认值工厂。

# defaultdict(lambda: defaultdict(int))：创建一个新的 defaultdict，其默认值是由上述 lambda 函数提供的。
# 当您访问该字典中不存在的键时，它将自动创建一个新的 defaultdict(int)。

# 使用嵌套的 defaultdict 可以简化代码，因为您不需要显式检查并创建键及其默认值。
# 在这个例子中，flight_map 字典用于存储从一个城市到另一个城市的航班数量。
# 当您尝试增加一个航班数量时，如果原始的航班数量不存在，嵌套的 defaultdict 会自动为您创建默认值为 0 的整数。



class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        graph = defaultdict(list)
        for f, t in tickets:
            graph[f].append(t)
        
        for f in graph:
            graph[f].sort()
        
        path = ["JFK"]
        
        def backtracking(start_point):
            if len(path) == len(tickets) + 1:
                return True
            
            for _ in graph[start_point]:
                end_point = graph[start_point].pop(0)
                path.append(end_point)
                if backtracking(end_point):
                    return True
                path.pop()
                graph[start_point].append(end_point)
        
        backtracking("JFK")
        return path