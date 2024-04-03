// 114_二叉树展开为链表
// https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/description/?envType=study-plan-v2&envId=top-100-liked

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
    void flatten(TreeNode *root)
    {
        vector<TreeNode *> l;
        preorderTraversal(root, l);
        int n = l.size();
        for (int i = 1; i < n; i++)
        {
            TreeNode *prev = l.at(i - 1), *curr = l.at(i);
            prev->left = nullptr;
            prev->right = curr;
        }
    }

    void preorderTraversal(TreeNode *root, vector<TreeNode *> &l)
    {
        if (root != NULL)
        {
            l.push_back(root);
            preorderTraversal(root->left, l);
            preorderTraversal(root->right, l);
        }
    }
};

class Solution
{
public:
    void flatten(TreeNode *root)
    {
        auto v = vector<TreeNode *>();
        auto stk = stack<TreeNode *>();
        TreeNode *node = root;
        while (node != nullptr || !stk.empty())
        {
            while (node != nullptr)
            {
                v.push_back(node);
                stk.push(node);
                node = node->left;
            }
            node = stk.top();
            stk.pop();
            node = node->right;
        }
        int size = v.size();
        for (int i = 1; i < size; i++)
        {
            auto prev = v.at(i - 1), curr = v.at(i);
            prev->left = nullptr;
            prev->right = curr;
        }
    }
};
