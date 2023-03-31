# 117_Populating_Next_Right_Pointers_in_Each_Node_II
# https://leetcode.cn/problems/populating-next-right-pointers-in-each-node-ii/

class Solution:
    def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if root is None:
            return root

        queue = deque([root])
        total = []
        
        while queue:
            level_length = len(queue)
            tmp = []
            
            for _ in range(level_length):
                node = queue.popleft()
                tmp.append(node)
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                
            total.append(tmp) 
            
        for vec in total:
            for i in range(len(vec)-1):
                vec[i].next = vec[i+1]
                
        return root