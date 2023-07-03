# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

from typing import Optional


# 反转链表 递归写法 ⭐
def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
    if head is None or head.next is None:
        return head
    new_head = self.reverseList(head.next)
    head.next.next = head  # 把下一个节点指向自己
    head.next = None  # 断开指向下一个节点的连接，保证最终链表的末尾节点的 next 是空节点
    return new_head


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


# 评论区题解
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head
        new_head = self.reverseList(head.next)
        head.next.next = head  # 把下一个节点指向自己
        head.next = None  # 断开指向下一个节点的连接，保证最终链表的末尾节点的 next 是空节点
        return new_head

    # l1 和 l2 为当前遍历的节点，carry 为进位
    def addTwo(self, l1: Optional[ListNode], l2: Optional[ListNode], carry=0) -> Optional[ListNode]:
        if l1 is None and l2 is None:  # 递归边界：l1 和 l2 都是空节点
            return ListNode(carry) if carry else None  # 如果进位了，就额外创建一个节点
        if l1 is None:  # 如果 l1 是空的，那么此时 l2 一定不是空节点
            l1, l2 = l2, l1  # 交换 l1 与 l2，保证 l1 非空，从而简化代码
        carry += l1.val + (l2.val if l2 else 0)  # 节点值和进位加在一起
        l1.val = carry % 10  # 每个节点保存一个数位
        l1.next = self.addTwo(l1.next, l2.next if l2 else None, carry // 10)  # 进位
        return l1

    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        l1 = self.reverseList(l1)
        l2 = self.reverseList(l2)  # l1 和 l2 反转后，就变成【2. 两数相加】了
        l3 = self.addTwo(l1, l2)
        return self.reverseList(l3)