# 99_Recover_Binary_Search_Tree
# https://leetcode.cn/problems/recover-binary-search-tree/


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
# class Solution:
#     def recoverTree(self, root: Optional[TreeNode]) -> None:
#         """
#         Do not return anything, modify root in-place instead.
#         """
#         self.p = TreeNode(-2**32)
#         self.q = TreeNode(2**32)
        
#         def predfs(node: TreeNode):
#             if node is None:
#                 return 
            
#             predfs(node.left)
#             if node.val < self.p.val:
#                 return
#             self.p = node
            
#             predfs(node.right)
            
#         def posdfs(node: TreeNode):
#             if node is None:
#                 return 
            
#             posdfs(node.right)
#             if node.val > self.q.val:
#                 return
#             self.p = node
            
#             posdfs(node.left)
            
#         predfs(root)
#         posdfs(root)
        
#         print(self.p)
#         print(self.q)
        
#         self.p, self.q = self.q, self.p    
# 主要更改如下：

# 不再使用 predfs 和 posdfs 函数，而是使用一个中序遍历函数 inorder_traversal。
# 删除了不必要的 self.p 和 self.q 变量，并添加了 self.first、self.second 和 self.prev 变量以跟踪需要交换的节点。
# 修改了 inorder_traversal 函数内的条件，以识别需要交换的两个节点。
# 在中序遍历后，交换 self.first 和 self.second 节点的值。
class Solution:
    def recoverTree(self, root: Optional[TreeNode]) -> None:
        self.first = None
        self.second = None
        
        self.prev = TreeNode(float('-inf'))
        
        def inorder_traversal(node: TreeNode):
            if not node:
                return
            
            inorder_traversal(node.left)
            
            if not self.first and self.prev.val >= node.val:
                self.first = self.prev
                
            if self.first and self.prev.val >= node.val:
                self.second = node
                
            self.prev = node
            
            inorder_traversal(node.right)
            
        inorder_traversal(root)
        
        self.first.val, self.second.val = self.second.val, self.first.val
        
        
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def recoverTree(self, root: TreeNode) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        firstNode = None
        secondNode = None
        
        pre = TreeNode(float("-inf"))

        stack = []
        p = root
        
        while p or stack:
            while p:
                stack.append(p)
                p = p.left
            p = stack.pop()
            
            if not firstNode and pre.val > p.val:
                firstNode = pre
                    
            if firstNode and pre.val > p.val:
                #print(firstNode.val,pre.val, p.val)
                secondNode = p
                
            pre = p
            p = p.right
            
        firstNode.val, secondNode.val = secondNode.val, firstNode.val


# class Solution {
# public:
#     void recoverTree(TreeNode* root) {
#         TreeNode *x = nullptr, *y = nullptr, *pred = nullptr, *predecessor = nullptr;

#         while (root != nullptr) {
#             if (root->left != nullptr) {
#                 // predecessor 节点就是当前 root 节点向左走一步，然后一直向右走至无法走为止
#                 predecessor = root->left;
#                 while (predecessor->right != nullptr && predecessor->right != root) {
#                     predecessor = predecessor->right;
#                 }
                
#                 // 让 predecessor 的右指针指向 root，继续遍历左子树
#                 if (predecessor->right == nullptr) {
#                     predecessor->right = root;
#                     root = root->left;
#                 }
#                 // 说明左子树已经访问完了，我们需要断开链接
#                 else {
#                     if (pred != nullptr && root->val < pred->val) {
#                         y = root;
#                         if (x == nullptr) {
#                             x = pred;
#                         }
#                     }
#                     pred = root;

#                     predecessor->right = nullptr;
#                     root = root->right;
#                 }
#             }
#             // 如果没有左孩子，则直接访问右孩子
#             else {
#                 if (pred != nullptr && root->val < pred->val) {
#                     y = root;
#                     if (x == nullptr) {
#                         x = pred;
#                     }
#                 }
#                 pred = root;
#                 root = root->right;
#             }
#         }
#         swap(x->val, y->val);
#     }
# };