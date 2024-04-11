# 337_打家劫舍_III
# https://leetcode.cn/problems/house-robber-iii/description/?envType=daily-question&envId=2023-09-18

# 小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为 root 。

# 除了 root 之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果 两个直接相连的房子在同一天晚上被打劫 ，房屋将自动报警。

# 给定二叉树的 root 。返回 在不触动警报的情况下 ，小偷能够盗取的最高金额 。

# Definition for a binary tree node.
from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# class Solution:
#     def rob(self, root: Optional[TreeNode]) -> int:
#         def dfs(node):
#             if not node:
#                 return 0, 0

#             l_rob, l_not_rob = dfs(node.left)
#             r_rob, r_not_rob = dfs(node.right)

#             rob = node.val + l_not_rob + r_not_rob
#             not_rob = max(l_rob + r_rob, l_rob + r_not_rob, r_rob + l_not_rob, r_not_rob + l_not_rob)

#             return rob, not_rob

#         return max(dfs(root))


class Solution:
    def rob(self, root: Optional[TreeNode]) -> int:
        def dfs(node):
            if not node:
                return 0, 0

            l_rob, l_not_rob = dfs(node.left)
            r_rob, r_not_rob = dfs(node.right)

            rob = node.val + l_not_rob + r_not_rob
            not_rob = max(l_not_rob, l_rob) + max(r_not_rob, r_rob)

            return rob, not_rob

        return max(dfs(root))
