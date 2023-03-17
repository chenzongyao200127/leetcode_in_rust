# 86_Partition_List
# https://leetcode.cn/problems/partition-list/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        small = ListNode(0)
        large = ListNode(0)
        large_first = large
        small_first = small
        p = head
        while p:
            if p.val < x:
                small.next = p
                small = small.next
            else:
                large.next = p
                large = large.next
            p = p.next
        large.next = None
        small.next = large_first.next
        return small_first.next

# 这段代码是一个名为Solution的类，其中包含一个名为partition的方法。该方法的目的是将给定链表head根据给定值x进行分区。
# 分区后，所有小于x的节点都应该在大于或等于x的节点之前。方法的返回值是分区后链表的头节点。

# 以下是代码的逐行解释：
# 创建两个新的链表头节点small和large，它们的初始值为0。
# 将large_first和small_first分别指向large和small，以便稍后连接两个链表。
# 将指针p初始化为输入链表的头节点head。
# 使用while循环遍历输入链表，直到p为None。
# 在循环内，检查p.val是否小于x。
# 如果p.val小于x，将small链表的下一个节点设置为p，然后将small指针向前移动一个节点。
# 否则，将large链表的下一个节点设置为p，然后将large指针向前移动一个节点。
# 在循环的末尾，将p更新为p.next，以便在链表中向前移动。
# 当循环结束时，将large链表的最后一个节点的next设置为None，以表示链表的结尾。
# 将small链表的最后一个节点的next设置为large_first.next，从而连接small和large链表。
# 返回small_first.next作为新链表的头节点。
# 这段代码的主要思路是创建两个链表，一个包含小于x的节点，另一个包含大于或等于x的节点。
# 然后将这两然后将这两个链表连接在一起，形成一个新的分区链表。这种方法保留了原始链表中节点的相对顺序。
# 在完成分区操作后，返回新链表的头节点。