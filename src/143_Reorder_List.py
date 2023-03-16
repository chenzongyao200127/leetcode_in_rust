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
        slow.next = None # 这一步别忘记了
        right = self.reverse_list(right)
        
        cur = head
        while right and cur:
            cur_sec = right
            right = right.next
            next_cur = cur.next
            cur_sec.next = cur.next
            cur.next = cur_sec
            cur = next_cur
        
    def reverse_list(self, head):
        previous = None
        current = head
        while current is not None:
            next_node = current.next
            current.next = previous
            previous = current
            current = next_node
        return previous

'''
反转错了
    1.快慢指针找到中点 
    2.拆成两个链表 
    3.遍历两个链表，后面的塞到前面的“缝隙里”
'''
# class Solution:
#     def reorderList(self, head: Optional[ListNode]) -> None:
#         """
#         Do not return anything, modify head in-place instead.
#         """
#         slow, fast = head, head
#         while fast and fast.next:
#             slow = slow.next
#             fast = fast.next.next
        
#         right = slow.next
#         slow.next = None
#         slow = self.reverse_list(head)

#         left_curr = slow.next
#         left_prev = slow
#         right_curr = right.next
#         right_prev = right
#         while right_prev:
#             left_prev.next = right_prev
#             left_prev = left_curr
#             left_curr = left_curr.next
#             right_prev.next = left_prev
#             right_prev = right_curr
#             if right_curr:
#                 right_curr = right_curr.next
        
#         self.reverse_list(slow)
        
#     def reverse_list(self, head):
#         previous = None
#         current = head
#         while current is not None:
#             next_node = current.next
#             current.next = previous
#             previous = current
#             current = next_node
#         return previous
        
        
# C++ , 不得不承认这个题挺好的，考察了很多链表的基本功
# 整体思路：
# 1、快慢指针找到中间节点
# 2、反转后半段链表
# 3、合并前半段链表和后半段链表
class Solution:
    def reorderList(self, head: ListNode) -> None:
        if not head:
            return
        
        mid = self.middleNode(head)
        l1 = head
        l2 = mid.next
        mid.next = None
        l2 = self.reverseList(l2)
        self.mergeList(l1, l2)
    
    def middleNode(self, head: ListNode) -> ListNode:
        slow = fast = head
        while fast.next and fast.next.next:
            slow = slow.next
            fast = fast.next.next
        return slow
    
    def reverseList(self, head: ListNode) -> ListNode:
        prev = None
        curr = head
        while curr:
            nextTemp = curr.next
            curr.next = prev
            prev = curr
            curr = nextTemp
        return prev

    def mergeList(self, l1: ListNode, l2: ListNode):
        while l1 and l2:
            l1_tmp = l1.next
            l2_tmp = l2.next

            l1.next = l2
            l1 = l1_tmp

            l2.next = l1
            l2 = l2_tmp