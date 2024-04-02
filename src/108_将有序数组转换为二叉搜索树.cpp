// 108_将有序数组转换为二叉搜索树
// https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/description/?envType=study-plan-v2&envId=top-100-liked

#include <vector>

// 这道题目要求我们将一个已经按升序排列的整数数组 `nums` 转换成一棵平衡二叉搜索树（BST）。
// 平衡二叉搜索树是一种特殊的二叉搜索树，其中每个节点的左右两个子树的高度差的绝对值不超过1，这可以确保树的操作（例如搜索、插入、删除）的效率。

// 为了建立这样一棵树，我们可以采用分治法的思想，递归地进行树的构建。以下是这个问题的详细思路：

// 1. **选择中间元素作为树的根**：由于数组是排序好的，我们可以将中间的元素作为BST的根节点，这样可以保证左子树的所有值都比根节点小，右子树的所有值都比根节点大。
// 2. **递归构建左子树**：选取中间元素左边的子数组，重复步骤1，将这部分数组转换为根节点的左子树。
// 3. **递归构建右子树**：选取中间元素右边的子数组，重复步骤1，将这部分数组转换为根节点的右子树。
// 4. **递归终止条件**：当子数组为空，即开始索引大于结束索引时，递归终止。

// ⭐
// 通过上述步骤，我们可以确保树是平衡的，因为每次都是从有序数组的中间分割，所以左右子树的节点数量最多相差1。
// 此外，由于我们总是选择中间元素作为根节点，因此树中的每个节点都符合二叉搜索树的性质。

// 举个例子，如果给定数组 `nums = [-10, -3, 0, 5, 9]`，初始的中间元素是 `0`，它将成为树的根节点。
// 然后，我们对左边的子数组 `[-10, -3]` 重复相同的过程，`-3` 将成为左子树的根节点，`-10` 将成为左子节点。
// 同样，右边的子数组 `[5, 9]` 也会进行相同的过程，`5` 成为右子树的根节点，`9` 成为右子节点。最终构建的平衡BST如下所示：

// ```
//     0
//    / \
//  -3   5
//  /     \
// -10     9
// ```

// 这棵树是平衡的，因为每个节点的左右子树高度差不超过1，并且它保持了二叉搜索树的性质，
// 即左子树中的所有值都小于根节点的值，右子树中的所有值都大于根节点的值。通过递归地应用这个过程，我们可以从有序数组构建出平衡的二叉搜索树。

using namespace std;
class TreeNode
{
public:
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution
{
public:
    TreeNode *sortedArrayToBST(vector<int> &nums)
    {
        return constructBST(nums, 0, nums.size() - 1);
    }

private:
    TreeNode *constructBST(vector<int> &nums, int left, int right)
    {
        if (left > right)
            return nullptr;

        // Use left + (right - left) / 2 to avoid potential integer overflow
        int mid = left + (right - left) / 2;

        // Create the current node with the middle element
        TreeNode *node = new TreeNode(nums[mid]);

        // Recursively construct the left and right subtrees
        node->left = constructBST(nums, left, mid - 1);
        node->right = constructBST(nums, mid + 1, right);

        return node;
    }
};
