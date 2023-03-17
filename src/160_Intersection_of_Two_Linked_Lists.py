# 160_Intersection_of_Two_Linked_Lists
# https://leetcode.cn/problems/intersection-of-two-linked-lists/

Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

# 直接暴力枚举，超时了        
class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        while headA:
            tmp = headB
            while tmp:
                if headA == tmp:
                    return headA
                tmp = tmp.next
            headA = headA.next
            
        return None
    

# 方法一：哈希集合
# 判断两个链表是否相交，可以使用哈希集合存储链表节点。
class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        tmp = headA
        visited = set()  # 修改为集合
        while tmp != None:
            visited.add(tmp)  # 使用 add 方法
            tmp = tmp.next
        tmp = headB
        while tmp != None:
            if tmp in visited:
                return tmp
        return None

# 双指针
# 很巧妙
class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        curA = headA
        curB = headB
        if (headA == null || headB == null) return null;
        while curA != curB:
            if curA == None:
                curA = headB
            else:
                curA = curA.next
            if curB == None:
                curB = headA
            else:
                curB = curB.next
        return curA

# Java
# public class Solution {
#     public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
#         if (headA == null || headB == null) return null;
#         ListNode pA = headA, pB = headB;
#         while (pA != pB) {
#             pA = pA == null ? headB : pA.next;
#             pB = pB == null ? headA : pB.next;
#         }
#         return pA;
#     }
# }




# C++
# class Solution {
# public:
#     ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
#         unordered_set<ListNode *> visited;
#         ListNode *temp = headA;
#         while (temp != nullptr) {
#             visited.insert(temp);
#             temp = temp->next;
#         }
#         temp = headB;
#         while (temp != nullptr) {
#             if (visited.count(temp)) {
#                 return temp;
#             }
#             temp = temp->next;
#         }
#         return nullptr;
#     }
# };