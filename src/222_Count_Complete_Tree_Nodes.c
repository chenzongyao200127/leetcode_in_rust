// 222_Count_Complete_Tree_Nodes

// 给你一棵 完全二叉树 的根节点 root ，求出该树的节点个数。
// 完全二叉树 的定义如下：在完全二叉树中，除了最底层节点可能没填满外，
// 其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。若最底层为第 h 层，则该层包含 1~ 2^h 个节点。

// Definition for a binary tree node.
#include <stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

struct TreeNode
{
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

int countNodes(struct TreeNode *root)
{
    int ans = 0;
    countTree(root, &ans);
    return ans;
}

void countTree(struct TreeNode *root, int *ans)
{
    if (root != NULL)
    {
        *ans += 1;
    }
    else
    {
        return;
    }

    countTree(root->right, ans);
    countTree(root->left, ans);
}

// function dfs(root) {
// 	if (满足特定条件）{
// 		// 返回结果 or 退出搜索空间
// 	}
//     dfs(root.left)
//     dfs(root.right)
// }

// 示例代码
int countNodes(struct TreeNode *root)
{
    if (root == 0)
        return 0;
    else if (root->left != NULL && root->right == NULL)
        return 1 + countNodes(root->left);
    else if (root->left == NULL && root->right != NULL)
        return 1 + countNodes(root->right);
    else
        return 1 + countNodes(root->left) + countNodes(root->right);
}

// 这样也可，因为不会出现第三种情况
int countNodes(struct TreeNode *root)
{
    if (root == 0)
        return 0;
    else if (root->left != NULL && root->right == NULL)
        return 1 + countNodes(root->left);
    // else if(root->left==NULL&&root->right!=NULL) return 1+countNodes(root->right);
    else
        return 1 + countNodes(root->left) + countNodes(root->right);
}

// 二分查找 + 位运算
int countNodes(struct TreeNode *root)
{
    if (root == NULL)
    {
        return 0;
    }

    int level = 0;
    struct TreeNode *node = root;
    while (node->left != NULL)
    {
        level += 1;
        node = node->left;
    }

    int low = 1 << level, high = (1 << (level + 1)) - 1;
    while (low < high)
    {
        int mid = (high - low + 1) / 2 + low;
        if (exists(root, level, mid))
        {
            low = mid;
        }
        else
        {
            high = mid - 1;
        }
    }

    return low;
}

bool exists(struct TreeNode *root, int level, int k)
{
    int bits = 1 << (level - 1);
    struct TreeNode *node = root;
    while (node != NULL && bits > 0)
    {
        if (!(bits & k))
        {
            node = node->left;
        }
        else
        {
            node = node->right;
        }
        bits >>= 1;
    }
    return node != NULL;
}
