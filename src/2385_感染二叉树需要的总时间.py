# 2385_感染二叉树需要的总时间
# https://leetcode.cn/problems/amount-of-time-for-binary-tree-to-be-infected/description/?envType=daily-question&envId=2024-04-24

from collections import deque, defaultdict
from collections import deque
from typing import Optional

# Definition for a binary tree node.


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        def dfs(node: Optional[TreeNode], parent: Optional[TreeNode]) -> None:
            if node is None:
                return

            self.parent[node] = parent

            if node.val == start:
                self.start_node = node

            dfs(node.left, node)
            dfs(node.right, node)

        def max_depth(node: Optional[TreeNode], parent: TreeNode) -> int:
            if node is None:
                return -1  # 注意这里是 -1,因为 start 的深度为 0

            neighbors = [node.left, node.right, self.parent[node]]
            return max(max_depth(neighbor, node) for neighbor in neighbors if neighbor != parent) + 1

        self.parent = {}
        self.start_node = None

        dfs(root, None)
        return max_depth(self.start_node, self.start_node)


class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        graph = defaultdict(list)

        queue = deque([root])
        while queue:
            node = queue.popleft()
            if node.left:
                queue.append(node.left)
                graph[node.val].append(node.left.val)
                graph[node.left.val].append(node.val)
            if node.right:
                queue.append(node.right)
                graph[node.val].append(node.right.val)
                graph[node.right.val].append(node.val)

        queue = deque([start])
        visited = {start}
        time = 0
        while queue:
            for _ in range(len(queue)):
                node = queue.popleft()
                for neighbor in graph[node]:
                    if neighbor not in visited:
                        visited.add(neighbor)
                        queue.append(neighbor)
            time += 1

        return time - 1
