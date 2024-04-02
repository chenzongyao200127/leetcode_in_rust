// 101_对称二叉树
// https://leetcode.cn/problems/symmetric-tree/description/?envType=study-plan-v2&envId=top-100-liked

#include <queue>

// struct TreeNode {
//     int val;
//     TreeNode *left;
//     TreeNode *right;
//     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
// };
#include <queue>

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution
{
public:
    bool isSymmetric(TreeNode *root)
    {
        if (!root)
            return true;

        std::queue<TreeNode *> q;
        q.push(root->left);
        q.push(root->right);

        while (!q.empty())
        {
            TreeNode *node1 = q.front();
            q.pop();
            TreeNode *node2 = q.front();
            q.pop();

            if (!node1 && !node2)
                continue;
            if (!node1 || !node2)
                return false;
            if (node1->val != node2->val)
                return false;

            q.push(node1->left);
            q.push(node2->right);
            q.push(node1->right);
            q.push(node2->left);
        }

        return true;
    }
};