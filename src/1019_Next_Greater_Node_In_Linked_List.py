# 1019_Next_Greater_Node_In_Linked_List
# https://leetcode.cn/problems/next-greater-node-in-linked-list/


# Definition for singly-linked list.
# 超时
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    
class Solution:
    def nextLargerNodes(self, head: Optional[ListNode]) -> List[int]:
        nums = []
        stack = []
        
        cur = head
        while cur:
            nums.append(cur.val)
            cur = cur.next
        n = len(nums)
        
        ans = [0] * n
        
        for i in range(n):
            while stack and nums[i] > nums[stack[-1]]:
                idx = stack.pop()
                ans[idx] = nums[i]
            stack.append(i)
            
        return ans 
    
        
        
# 超时    
class Solution:
    def nextLargerNodes(self, head: Optional[ListNode]) -> List[int]:
        res = []
        
        def nextLargeNode(node: ListNode):
            if node is None:
                return
            
            curr = node.next
            ans = node.val
            while curr:
                if curr.val > ans:
                    ans = curr.val
                    break
                curr = curr.next
            
            nonlocal res
            if ans > node.val:
                res.append(ans)
            else:
                res.append(0)
                
        while head:
            nextLargeNode(head)
            head = head.next
        
        return res