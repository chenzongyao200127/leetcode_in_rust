# https://leetcode.cn/problems/path-sum-iii/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# 二次递归            
class Solution:        
    def rootSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        if root == None:
            return 0
        
        ans = 0
        if root.val == targetSum:
            ans += 1
        ans += self.rootSum(root.left, targetSum - root.val)
        ans += self.rootSum(root.right, targetSum - root.val)
        return ans
    
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        if root == None:
            return 0
        ans = self.rootSum(root, targetSum)
        ans += self.pathSum(root.left, targetSum)
        ans += self.pathSum(root.right, targetSum)
        return ans


class Solution:
    def pathSum(self, root: TreeNode, targetSum: int) -> int:
        def rootSum(root, targetSum):
            if root is None:
                return 0

            ret = 0
            if root.val == targetSum:
                ret += 1

            ret += rootSum(root.left, targetSum - root.val)
            ret += rootSum(root.right, targetSum - root.val)
            return ret
        
        if root is None:
            return 0
            
        ret = rootSum(root, targetSum)
        ret += self.pathSum(root.left, targetSum)
        ret += self.pathSum(root.right, targetSum)
        return ret
# 作者：LeetCode-Solution
# 链接：https://leetcode.cn/problems/path-sum-iii/solution/lu-jing-zong-he-iii-by-leetcode-solution-z9td/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


# 前缀和
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        prefix = collections.defaultdict(int)
        prefix[0] = 1
        
        def dfs(root, curr) -> int:
            if root == None:
                return 0
            
            ret = 0
            curr += root.val
            ret += prefix[curr - targetSum]
            prefix[curr] += 1
            ret += dfs(root.left, curr)
            ret += dfs(root.right, curr)
            prefix[curr] -= 1
            
            return ret
        
        return dfs(root, 0)
            
            
            
            
            