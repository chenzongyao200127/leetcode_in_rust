# 109_Convert_Sorted_List_to_Binary_Search_Tree
# https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def sortedListToBST(self, head: Optional[ListNode]) -> Optional[TreeNode]:
        if head is None:
            return None
        
        def sortedArrayToBST(nums: List[int]) -> Optional[TreeNode]:
            if not nums:
                return None
            
            mid = len(nums) // 2
            root = TreeNode(nums[mid])
            
            root.left = sortedArrayToBST(nums[:mid])
            root.right = sortedArrayToBST(nums[mid + 1:])
            
            return root
        
        nums = []
        while head:
            nums.append(head.val)
            head = head.next
            
        root = sortedArrayToBST(nums)
        
        return root
    
    

class Solution:
    def sortedListToBST(self, head: Optional[ListNode]) -> Optional[TreeNode]:
        if not head:
            return None
        if not head.next:
            return TreeNode(head.val)
        
        pre = None
        slow = head
        fast = head
        while fast and fast.next:
            pre = slow
            slow = slow.next
            fast = fast.next.next
    
        pre.next = None
        root = TreeNode(slow.val)
        
        root.left = self.sortedListToBST(head)
        root.right = self.sortedListToBST(slow.next)
        
        return root

# 这是一个将有序链表转换为高度平衡二叉搜索树的解决方案。类似于前面的有序数组转换为二叉搜索树的问题，
# 这里的方法也是采用递归。不过这次，我们需要遍历链表以找到中间节点，而不是直接访问数组的中间元素。

# 这个解决方案中，sortedListToBST 函数接收一个单链表的头节点，然后返回对应的高度平衡的二叉搜索树。以下是代码的解释：

# 如果 head 为空，则返回 None。
# 如果链表只有一个节点，返回以该节点值为根节点的树。

# 使用快慢指针法找到链表的中间节点。fast 指针每次移动两个节点，slow 指针每次移动一个节点。
# 当 fast 指针到达链表末尾时，slow 指针将指向链表的中间节点。

# 将链表从中间节点断开，形成两个子链表。
# 用中间节点的值创建根节点。
# 递归地将左半部分链表转换为左子树，将右半部分链表转换为右子树。

# 这个解决方案能够在 O(nlogn) 的时间复杂度内完成有序链表到高度平衡二叉搜索树的转换。