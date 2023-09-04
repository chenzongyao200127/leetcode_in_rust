# 449_Serialize_and_Deserialize_BST
# https://leetcode.cn/problems/serialize-and-deserialize-bst/?envType=daily-question&envId=2023-09-04

# 序列化是将数据结构或对象转换为一系列位的过程，以便它可以存储在文件或内存缓冲区中，或通过网络连接链路传输，以便稍后在同一个或另一个计算机环境中重建。

# 设计一个算法来序列化和反序列化 二叉搜索树 。 对序列化/反序列化算法的工作方式没有限制。 您只需确保二叉搜索树可以序列化为字符串，并且可以将该字符串反序列化为最初的二叉搜索树。

# 编码的字符串应尽可能紧凑。

 

# 示例 1：
# 输入：root = [2,1,3]
# 输出：[2,1,3]

# 示例 2：
# 输入：root = []
# 输出：[]

# 二叉搜索树 (BST) 的一个重要特性是对于 BST 的任何一个节点，
# 其左子树的所有节点的值都小于该节点的值，其右子树的所有节点的值都大于该节点的值。

# 利用这一特性，我们可以使用前序遍历（根左右）的方式序列化 BST。在反序列化时，我们可以利用 BST 的特性重建整个树。

# 序列化
# 使用前序遍历将二叉搜索树转换为字符串：
# 访问根节点
# 访问左子树
# 访问右子树
# 每次访问一个节点，我们都将它添加到结果字符串中，并用逗号 , 分隔。

# 反序列化
# 使用逗号 , 分隔字符串以获取所有节点的值。
# 由于我们使用前序遍历，所以数组的第一个值是根。
# 对于每一个值，如果它小于当前节点，则它是当前节点的左孩子，否则是一个祖先节点的右孩子。


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

from typing import Optional
class Codec:
    s = ""

    def serialize(self, root: Optional[TreeNode]) -> str:
        """Encodes a tree to a single string.
        """
        tmp = []
        def dfs(node):
            if not node:
                return
            
            tmp.append(node.val)
            
            dfs(node.left)
            dfs(node.right)
        
        dfs(root)
        self.s = ','.join(tmp)
        
        return self.s

    def deserialize(self, data: str) -> Optional[TreeNode]:
        """Decodes your encoded data to tree.
        """
        if not data:
            return None
        
        values = list(map(int, data.split(',')))
        def constree(values, lower, upper):
            if not values or values[0] < lower or values[0] > upper:
                return None
            root_val = values.pop(0)
            root = TreeNode(root_val)
            root.left = constree(values, lower, root_val)
            root.right = constree(values, root_val, upper)
            return root
        
        res = constree(values, float('inf'), float('inf'))
        
        return res 


# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# tree = ser.serialize(root)
# ans = deser.deserialize(tree)
# return ans

