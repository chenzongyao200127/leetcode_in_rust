// 100_Same_Tree
// https://leetcode.cn/problems/same-tree/

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

bool isSameTree(struct TreeNode *p, struct TreeNode *q)
{
    if (p == NULL && q == NULL)
    {
        return true;
    }
    if (p != NULL && q == NULL)
    {
        return false;
    }
    if (p == NULL && q != NULL)
    {
        return false;
    }
    if (p->val == q->val)
    {
        return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
    }
    else
    {
        return false;
    }
}
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

bool isSameTree(struct TreeNode *p, struct TreeNode *q)
{
    struct TreeNode *t1 = p, *t2 = q;
    if (!t1 && !t2)
    {
        return true;
    }
    else if (!t1 || !t2)
    {
        return false;
    }
    else if (t1->val != t2->val)
    {
        return false;
    }
    else
    {
        return isSameTree(t1->left, t2->left) && isSameTree(t1->right, t2->right);
    }
}
