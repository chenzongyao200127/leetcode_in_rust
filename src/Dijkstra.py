import heapq
from collections import defaultdict, deque
from typing import List, Tuple, Dict


def dijkstra(graph: Dict[int, List[Tuple[int, int]]], start_vertex: int) -> Dict[int, int]:
    """
    使用Dijkstra算法找到从起始顶点到所有其他顶点的最短路径。

    参数:
    graph -- 图，表示为顶点到(邻接顶点, 边权重)列表的字典。
    start_vertex -- 起始顶点。

    返回:
    distances -- 从起始顶点到图中每个其他顶点的最短距离的字典。
    """
    # 初始化距离字典，所有距离设为无穷大
    distances = {vertex: float('infinity') for vertex in graph}
    distances[start_vertex] = 0
    # 优先队列，用于存储待处理的顶点和它们的临时距离
    priority_queue = [(0, start_vertex)]

    while priority_queue:
        # 从队列中获取当前距离最小的顶点
        current_distance, current_vertex = heapq.heappop(priority_queue)

        # 遍历当前顶点的邻接顶点
        for neighbor, weight in graph[current_vertex]:
            distance = current_distance + weight

            # 只有在找到更短的路径时才进行更新
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(priority_queue, (distance, neighbor))

    return distances


# 示例用法
if __name__ == "__main__":
    # 构建图: 边的格式是 (目标节点, 边的权重)
    graph = {
        0: [(1, 4), (2, 1)],
        1: [(3, 1)],
        2: [(1, 2), (3, 5)],
        3: [(4, 3)],
        4: []
    }

    # 计算从顶点0开始到所有顶点的最短路径
    distances = dijkstra(graph, 0)
    print("最短路径:", distances)
