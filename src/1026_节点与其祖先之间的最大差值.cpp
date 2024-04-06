// 1026_节点与其祖先之间的最大差值
// https://leetcode.cn/problems/maximum-difference-between-node-and-ancestor/description/

#include <vector>
#include <algorithm>
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
    int dfs(TreeNode *root, int mi, int ma)
    {
        if (root == nullptr)
        {
            return 0;
        }
        int diff = max(abs(root->val - mi), abs(root->val - ma));
        mi = min(mi, root->val);
        ma = max(ma, root->val);
        diff = max(diff, dfs(root->left, mi, ma));
        diff = max(diff, dfs(root->right, mi, ma));
        return diff;
    }

    int maxAncestorDiff(TreeNode *root)
    {
        return dfs(root, root->val, root->val);
    }
};
