# 147_对链表进行插入排序

from typing import Optional

# Definition for singly-linked list.


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def insertionSortList(self, head: ListNode) -> ListNode:
        if not head or not head.next:
            return head

        dummy = ListNode(0)
        dummy.next = head

        last_sorted = head
        current = head.next

        while current:
            if last_sorted.val <= current.val:
                last_sorted = last_sorted.next
            else:
                prev = dummy
                while prev.next.val <= current.val:
                    prev = prev.next

                last_sorted.next = current.next
                current.next = prev.next
                prev.next = current

            current = last_sorted.next

        return dummy.next
