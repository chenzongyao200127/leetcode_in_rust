// 199_二叉树的右视图
// https://leetcode.cn/problems/binary-tree-right-side-view/description/?envType=study-plan-v2&envId=top-100-liked

#include <memory>
#include <vector>
#include <queue>
#include <stack>

using namespace std;

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
    std::vector<int> rightSideView(TreeNode *root)
    {
        std::vector<int> res;
        if (!root)
            return res;

        std::queue<TreeNode *> q;
        q.emplace(root); // Using emplace for potential in-place construction

        while (!q.empty())
        {
            int size = q.size();
            // Iterate over all nodes at the current level
            for (int i = 0; i < size; ++i)
            {
                TreeNode *node = q.front();
                q.pop();

                if (node->left)
                    q.emplace(node->left);
                if (node->right)
                    q.emplace(node->right);

                // If it's the last node in the current level, add it to the results
                if (i == size - 1)
                    res.emplace_back(node->val);
            }
        }

        return res;
    }
};

class Solution
{
    vector<int> ans;

    void dfs(TreeNode *node, int depth)
    {
        if (node == nullptr)
            return;
        if (depth == ans.size())
            ans.push_back(node->val);
        dfs(node->right, depth + 1);
        dfs(node->left, depth + 1);
    }

public:
    vector<int> rightSideView(TreeNode *root)
    {
        dfs(root, 0);
        return ans;
    }
};