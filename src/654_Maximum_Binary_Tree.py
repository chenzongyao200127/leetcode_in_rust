# 654_Maximum_Binary_Tree
# https://leetcode.cn/problems/maximum-binary-tree/


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> Optional[TreeNode]:
        if not nums:
            return None
        max_val = max(nums)
        max_idx = nums.index(max_val)
        node = TreeNode(max_val)
        node.left = self.constructMaximumBinaryTree(nums[:max_idx])
        node.right = self.constructMaximumBinaryTree(nums[max_idx+1:])
        return node
    
# 可以通过将列表切片操作改为传递索引来优化代码性能。这样可以避免创建新的列表，从而减少内存使用。
# 同时，可以将列表的最大值和索引的查找操作合并为一步，以减少时间复杂度。下面是优化后的代码：
class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> Optional[TreeNode]:
        def construct(nums, l, r):
            if l > r:
                return None
            max_val = float('-inf')
            max_idx = -1
            for i in range(l, r+1):
                if nums[i] > max_val:
                    max_val = nums[i]
                    max_idx = i
            node = TreeNode(max_val)
            node.left = construct(nums, l, max_idx-1)
            node.right = construct(nums, max_idx+1, r)
            return node
        
        return construct(nums, 0, len(nums)-1)

# 这段代码实现了构建最大二叉树的功能。给定一个数组，每个元素都是唯一的，通过以下步骤构建最大二叉树：

# 1. 找到数组中的最大值，并将其作为根节点。
# 2. 将数组分成左右两部分，左边部分构建出来的子树挂在根节点的左侧，右边部分构建出来的子树挂在根节点的右侧。
# 3. 对于每个子问题（即对于每个子数组），递归执行上述过程。

# 具体实现方式如下：

# 首先创建一个空列表 `stk` 用于存储索引。然后遍历输入列表 `nums` 中所有元素，并为每个元素创建一个新结点并存储在 `tree` 列表中。
# 同时使用单调递减堆栈维护当前未处理区间内所有数值比当前数值小或相等的位置。

# 当我们遇到第 i 个数字时，如果它比堆顶位置所代表数字要小，
# 则说明这是堆顶所代表区间内某些数字向右扩展得到更长区间时产生决策错误导致被弹出了；
# 而如果它比堆顶位置所代表数字要大，则说明此时找到了该区间内真正应该作为父亲结点（即局部最大） 的那个数。
# 因此，我们将堆顶位置所代表的数字作为当前结点 i 的左孩子，并将 i 入栈。
# 如果当前堆栈非空，则说明前面还有一些数值比它小或相等，可以从中找到一个最大的数值作为其父亲结点。

# 接下来遍历 `nums` 中所有元素，对于每个元素都判断其在数组中的位置关系（即是否是某个区间内部分），
# 并按照上述规则构建出二叉树。最后返回整棵二叉树的根节点。

# 需要注意的是，在 Python 代码中使用了类型提示和 Optional 类型注解，
# 这要求输入参数 nums 是一个 List[int] 类型且输出结果可能为空（Optional[TreeNode]）。
        