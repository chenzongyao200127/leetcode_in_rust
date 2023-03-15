# 234_Palindrome_Linked_List
# https://leetcode.cn/problems/palindrome-linked-list/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        prev, curr = None, slow
        while curr:
            curr.next, prev, curr = prev, curr, curr.next
            
        p1, p2 = head, prev
        while p2:
            if p1.val != p2.val:
                return False
            p1, p2 = p1.next, p2.next
            
        return True
        

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
# 不考虑链表复原，快慢指针在寻找中间节点的过程中直接反转链表前半部分，找到中间节点之后直接从中间向两边开始比较
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        pre = None
        slow = fast = head
        while fast and fast.next:
            fast = fast.next.next
            next = slow.next
            slow.next = pre
            pre = slow
            slow = next
            
        if fast:
            slow = slow.next
            
        while slow:
            if slow.val != pre.val:
                return False
            pre = pre.next
            slow = slow.next
            
        return True





# 判断一个链表是否是回文的，可以使用以下步骤：
# 找到链表的中间节点，可以使用快慢指针的方法，快指针每次走两步，慢指针每次走一步，当快指针到达链表尾部时，慢指针刚好到达链表中间节点。
# 反转链表的后半部分，从中间节点开始，将后半部分的链表反转。
# 从链表头和反转后的链表头开始，依次比较每个节点的值，如果全部相等，则链表是回文的，否则不是。
# 恢复链表，将反转后的链表再次反转，以恢复原始链表。
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def isPalindrome(head: ListNode) -> bool:
    # 找到链表的中间节点
    slow, fast = head, head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    
    # 反转链表的后半部分
    prev, curr = None, slow
    while curr:
        curr.next, prev, curr = prev, curr, curr.next
    
    # 比较链表前半部分和反转后的链表前半部分的值
    p1, p2 = head, prev
    while p2:
        if p1.val != p2.val:
            return False
        p1, p2 = p1.next, p2.next
    
    # 恢复链表
    curr, prev = prev, None
    while curr:
        curr.next, prev, curr = prev, curr, curr.next
    
    return True


# 方法一：将值复制到数组中后用双指针法
class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        vals = []
        current_node = head
        while current_node is not None:
            vals.append(current_node.val)
            current_node = current_node.next
            
        return vals == vals[::-1]
    

# 方法二：递归
function print_values_in_reverse(ListNode head)
    if head is NOT null
        print_values_in_reverse(head.next)
        print head.val
        
class Solution:
    def isPalindrome(self, head: ListNode) -> bool:

        self.front_pointer = head

        def recursively_check(current_node=head):
            if current_node is not None:
                if not recursively_check(current_node.next):
                    return False
                if self.front_pointer.val != current_node.val:
                    return False
                self.front_pointer = self.front_pointer.next
            return True

        return recursively_check()

# 方法三：快慢指针
class Solution:

    def isPalindrome(self, head: ListNode) -> bool:
        if head is None:
            return True

        # 找到前半部分链表的尾节点并反转后半部分链表
        first_half_end = self.end_of_first_half(head)
        second_half_start = self.reverse_list(first_half_end.next)

        # 判断是否回文
        result = True
        first_position = head
        second_position = second_half_start
        while result and second_position is not None:
            if first_position.val != second_position.val:
                result = False
            first_position = first_position.next
            second_position = second_position.next

        # 还原链表并返回结果
        first_half_end.next = self.reverse_list(second_half_start)
        return result    

    def end_of_first_half(self, head):
        fast = head
        slow = head
        while fast.next is not None and fast.next.next is not None:
            fast = fast.next.next
            slow = slow.next
        return slow

    def reverse_list(self, head):
        previous = None
        current = head
        while current is not None:
            next_node = current.next
            current.next = previous
            previous = current
            current = next_node
        return previous