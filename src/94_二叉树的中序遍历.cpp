// 94_二叉树的中序遍历
// https://leetcode.cn/problems/binary-tree-inorder-traversal/description/?envType=study-plan-v2&envId=top-100-liked

#include <vector>
using namespace std;

/**
 * Definition for a binary tree node.
 */

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
public:
    std::vector<int> inorderTraversal(TreeNode *root)
    {
        std::vector<int> ans;
        inorderHelper(root, &ans);
        return ans;
    }

private:
    void inorderHelper(TreeNode *node, std::vector<int> *ans)
    {
        if (node != nullptr)
        {
            inorderHelper(node->left, ans);  // Traverse left subtree
            ans->push_back(node->val);       // Visit current node
            inorderHelper(node->right, ans); // Traverse right subtree
        }
    }
};