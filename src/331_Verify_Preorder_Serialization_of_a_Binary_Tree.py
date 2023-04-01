 # 331_Verify_Preorder_Serialization_of_a_Binary_Tree
 # https://leetcode.cn/problems/verify-preorder-serialization-of-a-binary-tree/
 
# 逆向栈
class Solution(object):
    def isValidSerialization(self, preorder):
        stack = []
        
        for node in preorder.split(','):
            stack.append(node)
            
            while len(stack) >= 3 and stack[-1] == stack[-2] == '#' and stack[-3] != '#':
                stack.pop(), stack.pop(), stack.pop()
                stack.append('#')
        return len(stack) == 1 and stack.pop() == '#'


# 槽位思想
class Solution(object):
    def isValidSerialization(self, preorder):
        nodes = preorder.split(',')
        diff = 1
        
        for node in nodes:
            diff -= 1
            
            if diff < 0:
                return False
            
            if node != '#':
                diff += 2
                
        return diff == 0
