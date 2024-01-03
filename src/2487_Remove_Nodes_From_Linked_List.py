# 2487_Remove_Nodes_From_Linked_List
# https://leetcode.cn/problems/remove-nodes-from-linked-list/

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def reverse(head):
            if not head or not head.next:
                return head
            
            new_head = reverse(head.next)
            head.next.next = head
            head.next = None
            
            return new_head
        
        head = reverse(head)
        
        cur_max = head.val
        cur_node = head
    
        while cur_node and cur_node.next:
            if cur_node.next.val >= cur_max:
                cur_max = cur_node.next.val
                cur_node = cur_node.next
            else:
                cur_node.next = cur_node.next.next
        
        return reverse(head)
    
        
            
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def reverse(head):
            prev = None
            current = head
            while current:
                next_node = current.next
                current.next = prev
                prev = current
                current = next_node
            return prev

        # Reverse the linked list
        head = reverse(head)

        # Initialize the variables for tracking the current max and iterating through the list
        cur_max = head.val if head else float('-inf')
        cur_node = head

        # Traverse the reversed list
        while cur_node and cur_node.next:
            if cur_node.next.val >= cur_max:
                cur_max = cur_node.next.val
                cur_node = cur_node.next
            else:
                # Skip the node if it's not greater than or equal to the current max
                cur_node.next = cur_node.next.next

        # Reverse the list again to restore the original order
        return reverse(head)
            
            
def reverse_recursive(head):
    if not head or not head.next:
        return head
    
    new_head = reverse_recursive(head.next)
    head.next.next = head
    head.next = None
    
    return new_head