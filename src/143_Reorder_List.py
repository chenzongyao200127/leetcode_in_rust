# 143. Reorder List
# https://leetcode.cn/problems/reorder-list/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        right = slow.next
        slow.next = None
        slow = self.reverse_list(head)
        left_curr = slow.next
        left_prev = slow
        right_curr = right.next
        right_prev = right
        while right_prev:
            left_prev.next = right_prev
            left_prev = left_curr
            left_curr = left_curr.next
            right_prev.next = left_prev
            right_prev = right_curr
            if right_curr:
                right_curr = right_curr.next
        
        self.reverse_list(slow)
        
    def reverse_list(self, head):
        previous = None
        current = head
        while current is not None:
            next_node = current.next
            current.next = previous
            previous = current
            current = next_node
        return previous
        