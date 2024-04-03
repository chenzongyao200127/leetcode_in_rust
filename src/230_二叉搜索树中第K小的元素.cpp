// 230_二叉搜索树中第K小的元素
// https://leetcode.cn/problems/kth-smallest-element-in-a-bst/description/?envType=study-plan-v2&envId=top-100-liked

#include <memory>
#include <vector>
#include <unordered_map>
#include <stack>

using namespace std;

// 使用智能指针定义 TreeNode，以便自动管理内存

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class MyBst
{
public:
    MyBst(TreeNode *root)
    {
        this->root = root;
        countNodeNum(root);
    }

    // 返回二叉搜索树中第k小的元素
    int kthSmallest(int k)
    {
        TreeNode *node = root;
        while (node != nullptr)
        {
            int left = getNodeNum(node->left);
            if (left < k - 1)
            {
                node = node->right;
                k -= left + 1;
            }
            else if (left == k - 1)
            {
                break;
            }
            else
            {
                node = node->left;
            }
        }
        return node->val;
    }

private:
    TreeNode *root;
    unordered_map<TreeNode *, int> nodeNum;

    // 统计以node为根结点的子树的结点数
    int countNodeNum(TreeNode *node)
    {
        if (node == nullptr)
        {
            return 0;
        }
        nodeNum[node] = 1 + countNodeNum(node->left) + countNodeNum(node->right);
        return nodeNum[node];
    }

    // 获取以node为根结点的子树的结点数
    int getNodeNum(TreeNode *node)
    {
        if (node != nullptr && nodeNum.count(node))
        {
            return nodeNum[node];
        }
        else
        {
            return 0;
        }
    }
};

class Solution
{
public:
    int kthSmallest(TreeNode *root, int k)
    {
        MyBst bst(root);
        return bst.kthSmallest(k);
    }
};

class Solution
{
public:
    int kthSmallest(TreeNode *root, int k)
    {
        stack<TreeNode *> stack;
        while (root != nullptr || stack.size() > 0)
        {
            while (root != nullptr)
            {
                stack.push(root);
                root = root->left;
            }
            root = stack.top();
            stack.pop();
            --k;
            if (k == 0)
            {
                break;
            }
            root = root->right;
        }
        return root->val;
    }
};
