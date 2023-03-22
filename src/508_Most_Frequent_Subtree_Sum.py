# 508_Most_Frequent_Subtree_Sum
# https://leetcode.cn/problems/most-frequent-subtree-sum/

# Definition for a binary tree node.
# 508. Most Frequent Subtree Sum
# Given the root of a binary tree, return the most frequent subtree sum. 
# If there is a tie, return all the values with the highest frequency in any order.
# The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node 
# (including the node itself).
 
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def findFrequentTreeSum(self, root: Optional[TreeNode]) -> List[int]:
        cnt = Counter()
        def dfs(root):
            if not root:
                return 0 
            sum = root.val + dfs(root.left) + dfs(root.right)
            cnt[sum] += 1
            return sum
        
        dfs(root)
        maxCnt = max(cnt.values())
        return [s for s, c in cnt.items() if c == maxCnt]