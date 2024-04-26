# 236_二叉树的最近公共祖先
# https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/description/?envType=study-plan-v2&envId=top-100-liked

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        def dfs(root, p, q):
            if not root:
                return None
            if root == p or root == q:
                return root
            left = dfs(root.left, p, q)
            right = dfs(root.right, p, q)
            if left and right:
                return root
            if left:
                return left
            if right:
                return right
            return None
        return dfs(root, p, q)


class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        if not root:
            return None

        if root == p or root == q:
            return root

        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)

        if left and right:
            return root

        if left:
            return left

        if right:
            return right

        return None


class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # 创建一个栈用于树的遍历
        stack = [root]

        # 创建一个字典来存储所有节点的父节点
        parent = {root: None}

        # 迭代，直到p和q的所有父节点都被记录
        while p not in parent or q not in parent:
            node = stack.pop()

            # 将节点的子节点添加到栈中，并记录其父节点
            if node.left:
                parent[node.left] = node
                stack.append(node.left)
            if node.right:
                parent[node.right] = node
                stack.append(node.right)

        # 创建一个集合来存储p的所有祖先
        ancestors = set()

        # 添加p及其所有祖先到集合中
        while p:
            ancestors.add(p)
            p = parent[p]

        # 检查q及其祖先，找到第一个出现在p的祖先集合中的节点
        while q not in ancestors:
            q = parent[q]

        return q
