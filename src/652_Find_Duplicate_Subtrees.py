# 652_Find_Duplicate_Subtrees
# https://leetcode.cn/problems/find-duplicate-subtrees/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
# class Solution:
#     def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
#         def bfs(node: TreeNode):
#             if node is None:
#                 return
            
#             queue = deque([node])
#             res = []
            
#             while queue:
#                 level_length = len(queue)
#                 tmp = []
                
#                 for _ in range(level_length):
#                     node = queue.popleft()
#                     if node is '#':
#                         tmp.append(None)
#                     else:
#                         tmp.append(node.val)
                        
#                     if node is not '#' and node.left:
#                         queue.append(node.left)
#                     else:
#                         queue.append('#')
                        
#                     if node is not '#' and node.right:
#                         queue.append(node.right)
#                     else:
#                         queue.append('#')
                
#                 res.append(tmp)
            
#             return res
        
#         trees = set()
#         ans = []
        
#         def dfs(node: TreeNode):
#             if node is None:
#                 return
            
#             tmp = bfs(node)
#             if tmp in trees:
#                 ans.append(tmp)
#             trees.add(tmp)
                
#             dfs(node.left)
#             dfs(node.right)
            
#         dfs(root)
#         return ans
                
            
# class Solution:
#     def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
#         def bfs(node: TreeNode):
#             if node is None:
#                 return

#             queue = deque([node])
#             res = []

#             while queue:
#                 level_length = len(queue)
#                 tmp = []

#                 for _ in range(level_length):
#                     node = queue.popleft()
#                     if node is None:
#                         tmp.append(None)
#                     else:
#                         tmp.append((node.val, id(node)))

#                     if node is not None and node.left:
#                         queue.append(node.left)
#                     else:
#                         queue.append(None)

#                     if node is not None and node.right:
#                         queue.append(node.right)
#                     else:
#                         queue.append(None)

#                 res.append(tmp)

#             return res

#         trees = set()
#         ans = []

#         def dfs(node: TreeNode):
#             if node is None:
#                 return

#             tmp = tuple(bfs(node))
#             if tmp in trees:
#                 ans.append(tmp)
#             trees.add(tmp)

#             dfs(node.left)
#             dfs(node.right)

#         dfs(root)
#         return ans
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def __init__(self):
        self.rec = {}   
        self.l = []
        
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        rec = {}
        lst = []
        
        def postorder(root):
            if root is None:
                return '' 

            l = postorder(root.left)
            r = postorder(root.right)
            
            s = '_'.join((l , r, str(root.val)))  
            
            # rec 
            if s not in rec:
                rec[s] = 1
            elif s in rec and rec[s] == 1:
                lst.append(root) 
                rec[s] += 1 
            elif s in rec and rec[s] != 1:
                rec[s] +=1

            return s 

        postorder(root)

        return lst  

            
class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        st = dict()
        ans = list()

        def dfs(root):
            if not root:
                return ""
            
            left = dfs(root.left)
            right = dfs(root.right)
            
            cur = "_".join((str(root.val), left, right))
            
            if cur not in st:
                st[cur] = 1
            else:
                st[cur] += 1
                
            if st[cur] == 2:
                ans.append(root)
                
            return cur
        
        dfs(root)
        return ans



class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        tree_dict = defaultdict()
        tree_dict.default_factory = tree_dict.__len__
        count = Counter()
        ans = []

        def postorder_traversal(node: TreeNode):
            if node is None:
                return 0

            left_id = postorder_traversal(node.left)
            right_id = postorder_traversal(node.right)
            
            tree_id = tree_dict[node.val, left_id, right_id]
            count[tree_id] += 1

            if count[tree_id] == 2:
                ans.append(node)

            return tree_id

        postorder_traversal(root)
        return ans

# 方法一：使用序列化进行唯一表示
class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        def dfs(node: Optional[TreeNode]) -> str:
            if not node:
                return ""
            
            serial = "".join([str(node.val), "(", dfs(node.left), ")(", dfs(node.right), ")"])
            if (tree := seen.get(serial, None)):
                repeat.add(tree)
            else:
                seen[serial] = node
            
            return serial
        
        seen = dict()
        repeat = set()

        dfs(root)
        return list(repeat)


# 方法二：使用三元组进行唯一表示
class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        def dfs(node: Optional[TreeNode]) -> int:
            if not node:
                return 0
            
            tri = (node.val, dfs(node.left), dfs(node.right))
            
            if tri in seen:
                (tree, index) = seen[tri]
                repeat.add(tree)
                return index
            else:
                nonlocal idx
                idx += 1
                seen[tri] = (node, idx)
                return idx
        
        idx = 0
        seen = dict()
        repeat = set()

        dfs(root)
        return list(repeat)
