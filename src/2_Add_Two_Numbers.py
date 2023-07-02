# 2_Add_Two_Numbers
# https://leetcode.cn/problems/add-two-numbers/

# You are given two non-empty linked lists representing two non-negative integers. 
# The digits are stored in reverse order, and each of their nodes contains a single digit. 
# Add the two numbers and return the sum as a linked list.

# You may assume the two numbers do not contain any leading zero, except the number 0 itself.

# 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
# 请你将两个数相加，并以相同形式返回一个表示和的链表。
# 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

# 输入：l1 = [2,4,3], l2 = [5,6,4]
# 输出：[7,0,8]
# 解释：342 + 465 = 807.

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        def listNode2list(list):
            res = []
            while list:
                res.append(list.val)
                list = list.next
            return res

        x = listNode2list(l1)
        y = listNode2list(l2)

        if len(x) < len(y):
            x, y = y, x
        ans = []
        ca = 0
        for i in range(len(x)):
            a = x[i]
            if i < len(y):
                b = y[i]
            else:
                b = 0
            tmp = (a + b + ca) % 10
            ca = (a + b + ca) // 10
            ans.append(tmp)
        if ca != 0:
            ans.append(ca)


        head = ListNode(ans[0])
        tmp = head
        for i in range(1, len(ans)):
            tmp.next = ListNode(ans[i])
            tmp = tmp.next

        return head
        
            
# 优化
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
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
    
    
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if not l1:
            return l2
        if not l2:
            return l1

        l1.val += l2.val    # 将两数相加，赋值给 l1 节点
        if l1.val >= 10:
            l1.next = self.addTwoNumbers(ListNode(l1.val // 10), l1.next)
            l1.val %= 10
        
        l1.next = self.addTwoNumbers(l1.next, l2.next)
        return l1