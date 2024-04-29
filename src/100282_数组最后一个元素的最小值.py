# https://leetcode.cn/contest/weekly-contest-395/problems/minimum-array-end/

from typing import List, Optional


class Solution:
    def minEnd(self, n: int, x: int) -> int:
        if n == 1:
            return x

        zero_bits = []
        bit_position = 0
        temp_x = x

        # 找出x中所有为0的位的位置
        while temp_x > 0:
            if temp_x & 1 == 0:
                zero_bits.append(bit_position)
            temp_x >>= 1
            bit_position += 1

        # 确保有足够的位可以操作，以处理较大的n
        while len(zero_bits) < 30:
            zero_bits.append(bit_position)
            bit_position += 1

        result = x
        increment = 1
        count = n - 1

        # 通过设置x中的0位为1，逐步增加x的值
        for z in zero_bits:
            if count == 0:
                break
            # 如果count的当前位是1，则在result的对应位上设置1
            if increment & count:
                result |= (1 << z)
                count -= increment  # 递减count，表示已经处理了一个有效的增量
            increment <<= 1

        return result


s = Solution()
ans = s.minEnd(623423, 82312)
print(ans)


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        def dfs(node, target, path):
            if not node:
                return 0

            path.append(node.val)
            count = 0
            temp = 0
            for i in range(len(path) - 1, -1, -1):
                temp += path[i]
                if temp == target:
                    count += 1

            count += dfs(node.left, target, path)
            count += dfs(node.right, target, path)
            path.pop()
            return count

        return dfs(root, targetSum, [])

# Definition for a binary tree node.


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder:
            return None

        root_val = preorder[0]
        root = TreeNode(root_val)
        root_index = inorder.index(root_val)

        root.left = self.buildTree(
            preorder[1:1 + root_index], inorder[:root_index])
        root.right = self.buildTree(
            preorder[1 + root_index:], inorder[root_index + 1:])

        return root

# 二叉树展开为链表


class Solution:
    def flatten(self, root: TreeNode) -> None:
        if not root:
            return

        self.flatten(root.left)
        self.flatten(root.right)

        left = root.left
        right = root.right

        root.left = None
        root.right = left

        p = root
        while p.right:
            p = p.right

        p.right = right

# 反转单链表


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if not head:
            return None

        prev = None
        current = head

        while current:
            temp = current.next
            current.next = prev
            prev = current
            current = temp

        return prev

# testcase
# 1->2->3->4->5->NULL
# 5->4->3->2->1->NULL


# 两数之和
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        num_to_index = {}
        for i, num in enumerate(nums):
            if target - num in num_to_index:
                return [num_to_index[target - num], i]
            num_to_index[num] = i
        return []
