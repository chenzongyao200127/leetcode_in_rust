# 501_Find_Mode_in_Binary_Search_Tree
# https://leetcode.cn/problems/find-mode-in-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        order = []
        
        # def predfs(node: TreeNode):
        #     if node is None:
        #         return 
            
        #     predfs(node.left)
            
        #     nonlocal order
        #     order.append(node.val)
            
        #     predfs(node.right)
        def morris_inorder_traversal(root: TreeNode):
            current = root
            nonlocal order
            
            while current is not None:
                # a. 如果 current 没有左子节点，输出 current 的值，并将 current 设置为其右子节点。
                if current.left is None:
                    order.append(current.val)
                    current = current.right
                # b. 如果 current 有左子节点，找到 current 的前驱节点。接下来有两种情况：
                else:
                    # Find the inorder predecessor of current
                    pre = current.left
                    while pre.right is not None and pre.right != current:
                        pre = pre.right

                    # Make current as the right child of its inorder predecessor
                    # i. 如果前驱节点的右子节点为 None，将前驱节点的右子节点指向 current，并将 current 设置为其左子节点。
                    if pre.right is None:
                        pre.right = current
                        current = current.left
                    # ii. 如果前驱节点的右子节点指向 current，说明左子树已经遍历完成。
                    # 此时，输出 current 的值，将前驱节点的右子节点设置为 None，并将 current 设置为其右子节点。
                    else:
                        # Revert the changes made in the 'if' part to restore the original tree
                        pre.right = None
                        order.append(current.val)
                        current = current.right

        def get_mode(node_list):
            from collections import Counter
            res = []
            counter = Counter(node_list)
            
            # 使用 most_common() 方法找到计数最多的元素及其计数。most_common(1) 返回一个包含一个元组的列表，
            # 元组的第一个元素是计数最多的元素，第二个元素是它的计数。我们将最大计数值存储在 max_value 中。
            max_value = counter.most_common(1)[0][1]
            
            for k, v in counter.most_common():
                if v >= max_value:
                    res.append(k)
                    
            return res

        morris_inorder_traversal(root)
        return get_mode(order)



# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def __init__(self):
        self.max_count = 1
        self.count = 1
        self.pre = None
        
    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        res =[]
        def find(cur,res):
            if not cur:
                return
            
            find(cur.left,res)
            
            if self.pre and self.pre.val == cur.val:
                self.count += 1
                
            else: self.count = 1
            
            if self.count == self.max_count:
                res.append(cur.val)
                
            if self.count > self.max_count:
                res.clear()
                self.max_count = self.count
                res.append(cur.val)
                
            self.pre = cur
            find(cur.right, res)
            
        find(root,res)
        
        return res


# Morris 中序遍历
# Morris 中序遍历是一种在不使用递归和额外存储空间的情况下遍历二叉树的方法。
# 它利用了二叉树中的空闲指针（即空的右子节点指针），在遍历过程中建立线索，以保持对遍历顺序的跟踪。
# Morris 中序遍历的核心思想是将当前节点的前驱节点的右子节点指向当前节点，从而在遍历完左子树后能够回到当前节点。

# Morris 中序遍历的步骤如下：
# 初始化当前节点 current 为根节点。
# 当 current 不为 None 时：
# a. 如果 current 没有左子节点，输出 current 的值，并将 current 设置为其右子节点。
# b. 如果 current 有左子节点，找到 current 的前驱节点（即 current 左子树中值最大的节点）。接下来有两种情况：
#   i. 如果前驱节点的右子节点为 None，将前驱节点的右子节点指向 current，并将 current 设置为其左子节点。
#   ii. 如果前驱节点的右子节点指向 current，说明左子树已经遍历完成。此时，输出 current 的值，将前驱节点的右子节点设置为 None，并将 current 设置为其右子节点。
# 重复步骤 2，直到 current 为 None，遍历结束。
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def morris_inorder_traversal(root: TreeNode):
    current = root
    result = []
    
    while current is not None:
        # a. 如果 current 没有左子节点，输出 current 的值，并将 current 设置为其右子节点。
        if current.left is None:
            result.append(current.val)
            current = current.right
        # b. 如果 current 有左子节点，找到 current 的前驱节点（即 current 左子树中值最大的节点）。接下来有两种情况：
        else:
            # Find the inorder predecessor of current
            pre = current.left
            while pre.right is not None and pre.right != current:
                pre = pre.right

            # Make current as the right child of its inorder predecessor
            # i. 如果前驱节点的右子节点为 None，将前驱节点的右子节点指向 current，并将 current 设置为其左子节点。
            if pre.right is None:
                pre.right = current
                current = current.left
            # ii. 如果前驱节点的右子节点指向 current，说明左子树已经遍历完成。
            # 此时，输出 current 的值，将前驱节点的右子节点设置为 None，并将 current 设置为其右子节点。
            else:
                # Revert the changes made in the 'if' part to restore the original tree
                pre.right = None
                result.append(current.val)
                current = current.right
                
        def get_mode(node_list):
            from collections import Counter
            res = []
            counter = Counter(node_list)
            
            # 使用 most_common() 方法找到计数最多的元素及其计数。most_common(1) 返回一个包含一个元组的列表，
            # 元组的第一个元素是计数最多的元素，第二个元素是它的计数。我们将最大计数值存储在 max_value 中。
            max_value = counter.most_common(1)[0][1]
            
            for k, v in counter.most_common():
                if v >= max_value:
                    res.append(k)
                    
            return res