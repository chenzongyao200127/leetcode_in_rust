# 675_Cut_Off_Trees_for_Golf_Event
# https://leetcode.cn/problems/cut-off-trees-for-golf-event/

# 你被请来给一个要举办高尔夫比赛的树林砍树。树林由一个 m x n 的矩阵表示， 在这个矩阵中：

# 0 表示障碍，无法触碰
# 1 表示地面，可以行走
# 比 1 大的数 表示有树的单元格，可以行走，数值表示树的高度
# 每一步，你都可以向上、下、左、右四个方向之一移动一个单位，如果你站的地方有一棵树，那么你可以决定是否要砍倒它。

# 你需要按照树的高度从低向高砍掉所有的树，每砍过一颗树，该单元格的值变为 1（即变为地面）。

# 你将从 (0, 0) 点开始工作，返回你砍完所有树需要走的最小步数。 如果你无法砍完所有的树，返回 -1 。

# 可以保证的是，没有两棵树的高度是相同的，并且你至少需要砍倒一棵树。

from collections import defaultdict, deque
from typing import List
from pprint import pprint

# 这个问题可以用优先队列+广度优先搜索（BFS）来解决。我们首先遍历整个矩阵，
# 把所有的树（即大于1的值）以及它们的位置放到一个优先队列中，并按树的高度进行排序。

# 然后，我们从(0, 0)开始，每次都从优先队列中取出一棵树（即队首元素，因为队列已经按树的高度排序了）。
# 然后我们使用BFS找到当前位置到这棵树的最短路径，把路径长度累加到结果中，并把当前位置移动到这棵树的位置。

# 如果在某一步我们找不到到达下一棵树的路径（即BFS返回-1），那么我们就返回-1表示无法砍完所有的树。
# 否则，在砍完所有的树后，我们返回结果即为所求的最小步数。

# 这个问题的主要难点在于，我们需要在每一步都找到当前位置到下一棵树的最短路径，这需要我们在每一步都进行一次BFS。
# 同时，由于我们需要按照树的高度顺序砍树，所以我们需要对所有的树进行排序，这就需要使用优先队列。

# 下面是解决这个问题的Python代码：

from typing import List

def cutOffTree(forest: List[List[int]]) -> int:
    trees = sorted((v, r, c) for r, row in enumerate(forest) for c, v in enumerate(row) if v > 1)
    sr = sc = ans = 0

    def bfs(sr, sc, tr, tc):
        visited = [[False] * len(forest[0]) for _ in forest]
        visited[sr][sc] = True
        queue = [(sr, sc, 0)]
        for r, c, d in queue:
            if r == tr and c == tc:
                return d
            for nr, nc in ((r-1, c), (r+1, c), (r, c-1), (r, c+1)):
                if 0 <= nr < len(forest) and 0 <= nc < len(forest[0]) and not visited[nr][nc] and forest[nr][nc] != 0:
                    queue.append((nr, nc, d+1))
                    visited[nr][nc] = True
        return -1

    for _, tr, tc in trees:
        d = bfs(sr, sc, tr, tc)
        if d < 0: return -1
        ans += d
        sr, sc = tr, tc

    return ans


# A* 算法
# 由于问题的本质是求最短路，同时原问题的边权为 1，因此套用其他复杂度比 BFS 高的最短路算法，
# 对于本题而言是没有意义，但运用启发式搜索 AStar 算法来优化则是有意义。

# 因为在 BFS 过程中，我们会无差别往「四联通」方向进行搜索，直到找到「当前树点的下一个目标位置」为止，
# 而实际上，两点之间的最短路径往往与两点之间的相对位置相关。

# 举个 🌰，当前我们在位置 S，我们目标位置是 T，而 T 在 S 的右下方，
# 此时我们应当优先搜索方向"往右下方"的路径，当无法从"往右下方"的路径到达 T，我们再考虑搜索其他大方向的路径：

# 如何设计这样带有优先级的搜索顺序，则是 AStar 算法「启发式函数」的设计过程，其本质是对应了对「最小步数」的估算，
# 只有当我们确保「最小步数估算 <= 实际最小步数」，AStar 算法的正确性才得以保证。

# 因此我们往往会直接使用「理论最小步数」来作为启发式函数的，对于本题，可直接使用「曼哈顿距离」作为「理论最小步数」。

# 因此，如果我们是要从源点 S 到汇点 T，并且当前位于中途点 x 的话，点 x 的最小步数估算包括两部分：
# 到点 x 的实际步数 + 从点 x 到点 T 的理论最小步数（曼哈顿距离）。
# 使用「优先队列」按照「总的最小步数估算」进行出队，即可实现 AStar 算法的搜索顺序。

# AStar 算法做过很多次了，相关合集可以在 这里 看到。 另外，网上很多对 AStar 正确性证明不了解的人，
# 会缺少以下 map.get(nidx) > step + 1 判断逻辑。 
# 简单来说，启发式函数的设计是针对汇点而言的，
# 因此 AStar 算法搜索过程只确保对 T 的出入队次序能够对应回到点 T 第 k 短路，
# 而对于其余点的出入队次序到其余点的最短路没有必然的对应关系，因此当某个点的最小步数被更新，我们是要将其进行再次入队的。

# https://keson96.github.io/2016/08/02/2016-08-02-Heuristic-Search-Methods/



# 方法二：Dijkstra 算法
# 思路与算法

# 我们还可以利用 Dijkstra 算法求矩阵中两点的最短距离，Dijkstra 算法也是利用的广度优先搜索，
# 不同的是，每次对队列中优先选择最短路径的元素。visited 记录在某个时间点已经添加到队列中的节点，这些节点已被处理或在等待处理的队列中。
# 每次从队列中取出当前从起点开始的最少步数的点，对于下一个要处理的每个节点，查看他们的四个方向上相邻的点，如果相邻的点没有被遍历过且不是障碍，
# 将其加入到队列中，直到找到终点为止，返回当前的步数即可。最终返回所有的步数之和即为最终结果。 
# 使用该算法需要考虑的问题：由于题目中遇到障碍物无法通行的，因此当前选择的最短路径的节点并不是最优的，所以该解法在此题中性能不太好。


# Dijkstra算法和广度优先搜索（BFS）都是用于图的搜索算法，但它们之间有一些关键的区别。

# 广度优先搜索（BFS） 是一种用于遍历或搜索树或图的算法。
# 在图中，BFS优先搜索当前顶点的所有邻接顶点，然后再对这些顶点的邻接顶点进行搜索。
# 在BFS中，所有边的权重都被认为是相等的。


# Dijkstra算法 是一种用于寻找给定图中的最短路径的算法。相对于BFS，Dijkstra算法考虑了边的权重。
# 因此，Dijkstra算法可以用于解决BFS无法解决的问题，比如寻找加权图中的最短路径。

# 以下是Dijkstra算法的详细步骤：
# 初始化：选择一个源顶点，将其距离设为0。将所有其他顶点的距离设为无穷大（代表当前我们不知道到达这个顶点的最短路径）。
# 对于源顶点的每个邻接点，更新它们到源顶点的距离（如果新的距离更小的话）。
# 选择一个未访问过的具有最小距离的顶点，然后对其进行访问。这意味着我们将“永久”地确定该顶点的最短距离。
# 对于新选择的顶点的每个邻接点，检查是否可以通过新选择的顶点到达邻接点的距离更小。如果可以，更新它们的距离。
# 重复步骤3和4，直到所有的顶点都被访问过。最后的结果就是源顶点到所有其他顶点的最短路径。

# 举个例子，假设我们有以下的图，顶点A是源顶点。
#     A
#    / \
#  1/   \5
#  /     \
# B-------C
#    2

# 这个图中有三个顶点（A，B，C）和三条边，每条边的权重分别为1，5和2。

# 首先，我们将顶点A的距离设为0，将顶点B和C的距离设为无穷大。然后，我们更新顶点B和C的距离，变为1（A-B）和5（A-C）。
# 接下来，我们选择距离最小的未访问过的顶点，也就是顶点B，然后更新B的邻接点（顶点C）的距离。
# 因为通过B到C的距离（1+2=3）小于直接从A到C的距离（5），所以我们将C的距离更新为3。
# 最后，我们访问最后一个未访问过的顶点C。这样我们就找到了从A到B和C的最短路径，分别为1和3。

import heapq

def dijkstra(graph, start):
    distances = {node: float('infinity') for node in graph}
    distances[start] = 0
    queue = [(0, start)]

    while queue:
        current_distance, current_node = heapq.heappop(queue)

        if current_distance > distances[current_node]:
            continue

        for neighbor, weight in graph[current_node].items():
            distance = current_distance + weight

            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(queue, (distance, neighbor))

    return distances

# 示例图
graph = {
    'A': {'B': 1, 'C': 5},
    'B': {'A': 1, 'C': 2},
    'C': {'B': 2, 'A': 5}
}

print(dijkstra(graph, 'A'))  # 输出：{'A': 0, 'B': 1, 'C': 3}

# heapq.heappush(heap, item)
# 将 item 的值加入 heap 中，保持堆的不变性。

# heapq.heappop(heap)
# 弹出并返回 heap 的最小的元素，保持堆的不变性。如果堆为空，抛出 IndexError 。使用 heap[0] ，可以只访问最小的元素而不弹出它。