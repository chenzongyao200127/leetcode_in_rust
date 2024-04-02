// 98_验证二叉搜索树
// https://leetcode.cn/problems/validate-binary-search-tree/description/?envType=study-plan-v2&envId=top-100-liked

#include <limits.h>

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
    bool isValidBST(TreeNode *root)
    {
        return isValidBSTHelper(root, LONG_MIN, LONG_MAX);
    }

private:
    bool isValidBSTHelper(TreeNode *node, long long lower, long long upper)
    {
        if (node == nullptr)
        {
            return true;
        }

        int val = node->val;

        // 检查当前节点的值是否在合法范围内
        if (val <= lower || val >= upper)
        {
            return false;
        }

        // 递归验证左子树和右子树，并更新范围
        return isValidBSTHelper(node->left, lower, val) && isValidBSTHelper(node->right, val, upper);
    }
};