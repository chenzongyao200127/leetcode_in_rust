# 979_Distribute_Coins_in_Binary_Tree
# https://leetcode.cn/problems/distribute-coins-in-binary-tree/description/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# 查看题解（思路完全想不到）    
# 问：这种思路的本质是什么？

# 答：横看成岭侧成峰，每枚硬币移动的路径长度并不好计算，但是把这些路径叠起来，转换成每条边经过了多少枚硬币，就容易计算了（如下图）。


from typing import Optional
class Solution:
    def distributeCoins(self, root: Optional[TreeNode]) -> int:
        def dfs(node):
            nonlocal ans
            if not node:
                return 0,0
            
            cl, nl = dfs(node.left)
            cr, nr = dfs(node.right)
            cc = cl+cr+node.val
            nc = nl+nr+1
            ans += abs(cc - nc)
            return cc, nc
        
        ans = 0
        dfs(root)
        return ans
    
    

class Solution:
    def distributeCoins(self, root: Optional[TreeNode]) -> int:
        ans = 0
        def dfs(node: Optional[TreeNode]) -> int:
            if node is None:
                return 0
            d = dfs(node.left) + dfs(node.right) + node.val - 1
            nonlocal ans
            ans += abs(d)
            return d
        dfs(root)
        return ans

# class Solution {
# public:
#     int ret = 0;
#     int dfs(TreeNode* root) {
#         if (root == nullptr) return 0;
#         int left = dfs(root -> left);
#         int right = dfs(root -> right);
#         ret += abs(left) + abs(right);
#         return root -> val + (left + right) - 1;
#     }
#     int distributeCoins(TreeNode* root) {
#         dfs(root);
#         return ret;
#     }
# };

        
            
            
            