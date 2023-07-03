# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

from typing import Optional

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # 反转链表
        def reverseList(head):
            if not head:
                return head

            pre = None
            while head:
                next = head.next
                head.next = pre
                pre = head
                head = next
            return pre

        l1 = reverseList(l1)
        l2 = reverseList(l2)

        # 2 两数之和
        def addTwoRevList(l1, l2):
            carry = 0
            head = ListNode(0)
            current = head
            
            while l1 or l2 or carry:
                sum = 0
                if l1:
                    sum += l1.val
                    l1 = l1.next
                if l2:
                    sum += l2.val
                    l2 = l2.next
                sum += carry
                carry = sum // 10
                current.next = ListNode(sum % 10)
                current = current.next
            
            return head.next
        
        res = addTwoRevList(l1, l2)
        res = reverseList(res)
        
        return res
        
        
# 如果输入链表不能翻转该如何解决？使用栈模拟
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # s - stack
        s1, s2 = [], []
        while l1:
            s1.append(l1.val)
            l1 = l1.next
        while l2:
            s2.append(l2.val)
            l2 = l2.next
        res = None
        carry = 0
        while s1 or s2 or carry != 0:
            a = 0 if not s1 else s1.pop()
            b = 0 if not s2 else s2.pop()
            cur = a + b + carry
            carry = cur // 10
            cur %= 10
            cur_node = ListNode(cur)
            cur_node.next = res
            res = cur_node
        return res
