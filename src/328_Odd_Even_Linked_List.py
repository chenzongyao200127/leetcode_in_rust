# 328_Odd_Even_Linked_List
# https://leetcode.cn/problems/odd-even-linked-list/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head == None or head.next == None or head.next.next == None:
            return head
        
        headA = head
        headB = head.next
        curA = headA.next.next
        curB = headB.next.next
        headA.next = curA
        headB.next = curB
        while curA and curB:
            if curA.next == None or curB.next == None:
                break
            curA.next = curA.next.next
            curB.next = curB.next.next
            curA = curA.next
            curB = curB.next

        curA.next = headB
        return headA
    
    
# 示例代码
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head == None or head.next == None:
            return head
        
        temp1 = head
        temp2 = head.next
        two = head.next
        pre = None
        while head != None and temp2 != None:
            pre = head
            head.next = temp2.next
            head = head.next
            if head != None:
                temp2.next = head.next
                temp2 = temp2.next
        if head != None:
            head.next = two
        else:
            pre.next = two
        return temp1