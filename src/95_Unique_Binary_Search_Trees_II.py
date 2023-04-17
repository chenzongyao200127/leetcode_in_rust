# 95_Unique_Binary_Search_Trees_II
# https://leetcode.cn/problems/unique-binary-search-trees-ii/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def generateTrees(self, n: int) -> List[Optional[TreeNode]]:
        if n == 0:
            return []
        
        def build_trees(start, end):
            if start > end:
                return [None]
            
            all_trees = []
            for i in range(start, end+1):
                left_trees = build_trees(start, i - 1)
                right_trees = build_trees(i + 1, end)
                for l in left_trees:
                    for r in right_trees:
                        curr_tree = TreeNode(i)
                        curr_tree.left = l
                        curr_tree.right = r
                        all_trees.append(curr_tree)

            return all_trees
        
        return build_trees(1, n)
                

class Solution:
    def generateTrees(self, n: int) -> List[Optional[TreeNode]]:
        if n < 1:
            return []
        
        def build_tree(start, end):
            all_tree = []
            if start > end:
                return [None]
            
            for i in range(start, end + 1):
                left_tree = build_tree(start, i - 1)
                right_tree = build_tree(i + 1, end)

                # 从左子树集合中选出一棵左子树，从右子树集合中选出一棵右子树，拼接到根节点上
                for l in left_tree:
                    for r in right_tree:
                        cur_node = TreeNode(i)
                        cur_node.left = l
                        cur_node.right = r
                        all_tree.append(cur_node)
                        
            return all_tree
        
        res = build_tree(1, n)
        return res