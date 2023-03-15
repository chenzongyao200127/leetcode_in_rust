# 142_Linked_List_Cycle_II
# https://leetcode.cn/problems/linked-list-cycle-ii/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None
        
# 错误解答: 跳过了初始的比较
class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast, slow = head, head
        while fast and fast.next:
            fast = fast.next.next
            slow = slow.next
            if fast == slow:
                fast = head
                while True:
                    fast = fast.next
                    slow = slow.next
                    if fast == slow:
                        return fast        
        return None


class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast, slow = head, head
        while True:
            if not (fast and fast.next): return
            fast, slow = fast.next.next, slow.next
            if fast == slow: break
        fast = head
        while fast != slow:
            fast, slow = fast.next, slow.next
        return fast
    
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow = head
        fast = head
        while fast: # 因为fast比slow快, 如果不存在环, 只需判断fast是否为空即可
            slow = slow.next
            if fast.next == None: return None
            fast = fast.next.next
            if slow == fast: break # 不能写到while循环的判断条件中, 因为初始slow=head,会无法进入循环, java可以用do{}while;
        if fast == None: return None # 因为fast比slow快, 如果不存在环, 只需判断fast是否为空即可
        slow = head
        while slow != fast:
            slow = slow.next
            fast = fast.next
        return slow


    
class Solution(object):
    def detectCycle(self, head):
        fast, slow = head, head
        while True:
            if not (fast and fast.next): return
            fast, slow = fast.next.next, slow.next
            if fast == slow: break
        fast = head
        while fast != slow:
            fast, slow = fast.next, slow.next
        return fast
        
        
# public class Solution {
#     public ListNode detectCycle(ListNode head) {
#         ListNode pos = head;
#         Set<ListNode> visited = new HashSet<ListNode>();
#         while (pos != null) {
#             if (visited.contains(pos)) {
#                 return pos;
#             } else {
#                 visited.add(pos);
#             }
#             pos = pos.next;
#         }
#         return null;
#     }
# }