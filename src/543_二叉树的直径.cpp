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
    int diameterOfBinaryTree(TreeNode *root)
    {
        int ans = 0;
        diameterOfBinaryTreeDFS(root, ans);
        return ans;
    }

private:
    int diameterOfBinaryTreeDFS(TreeNode *node, int &ans)
    {
        if (node == nullptr)
            return 0;                                          // 对于 nullptr，返回的深度为 0
        int l_len = diameterOfBinaryTreeDFS(node->left, ans);  // 左子树的深度
        int r_len = diameterOfBinaryTreeDFS(node->right, ans); // 右子树的深度
        ans = max(ans, l_len + r_len);                         // 更新最大直径
        return max(l_len, r_len) + 1;                          // 返回节点的最大深度
    }
};
