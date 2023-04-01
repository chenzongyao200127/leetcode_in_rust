# 606_Construct_String_from_Binary_Tree
# https://leetcode.cn/problems/construct-string-from-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
# class Solution:
#     def tree2str(self, root: Optional[TreeNode]) -> str:
#         ans = []
        
#         def dfs(node: TreeNode):
#             if node is None:
#                 ans.append("(")
#                 ans.append(")")
#                 return
            
#             ans.append("(")
#             ans.append(str(node.val))
            
#             dfs(node.left)
#             dfs(node.right)
            
#             ans.append(")")
            
#         dfs(root)
#         res = ""
#         for i in range(1, len(ans)-1):
#             res = res + ans[i]
            
#         return res
            
            
class Solution:
    def tree2str(self, root: Optional[TreeNode]) -> str:
        if root is None:
            return ""
        
        if root.left is None and root.right is None:
            return str(root.val)
        
        if root.right is None:
            return f"{root.val}({self.tree2str(root.left)})"
        
        return f"{root.val}({self.tree2str(root.left)})({self.tree2str(root.right)})"
    
    
class Solution:
    def tree2str(self, root: Optional[TreeNode]) -> str:
        ans = ""
        st = [root]
        vis = set()
        
        while st:
            node = st[-1]
            
            if node in vis:
                if node != root:
                    ans += ")"
        
                st.pop()
            else:
                vis.add(node)
                if node != root:
                    ans += "("
                    
                ans += str(node.val)
                
                if node.left is None and node.right:
                    ans += "()"
                    
                if node.right:
                    st.append(node.right)
                if node.left:
                    st.append(node.left)
                    
        return ans
    
# 作者：LeetCode-Solution
# 链接：https://leetcode.cn/problems/construct-string-from-binary-tree/solution/gen-ju-er-cha-shu-chuang-jian-zi-fu-chua-e1af/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。