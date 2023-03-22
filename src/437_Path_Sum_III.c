// 437_Path_Sum_III
// https://leetcode.cn/problems/path-sum-iii/

// Definition for a binary tree node.
#include<stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

int rootSum(struct TreeNode* root, long long targetSum) {
    if (root == NULL) {
        return 0;
    }

    long long ans = 0;
    if (root->val == targetSum) {
        ans += 1;
    }

    ans += rootSum(root->left, targetSum - root->val);
    ans += rootSum(root->right, targetSum - root->val);
    return ans;
}


long long pathSum(struct TreeNode* root, long long targetSum){
    if (root == NULL) {
        return 0;
    }

    long long ans = rootSum(root, targetSum);
    ans += pathSum(root->left, targetSum);
    ans += pathSum(root->right, targetSum);

    return ans;
}


/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
//递归遍历树节点，判断是否为有效路径
int dfs(struct TreeNode * root, int targetSum,long sum)
{
    if(root == NULL)//叶子结点返回
    {
        return 0;
    }
    sum += (long)root->val;//统计节点值
    if(sum == targetSum)//是一个有效路径 +1
    {
        //再进行判断，以本节点还有没有有效路径
        return 1 + dfs(root->left, targetSum, sum) + dfs(root->right, targetSum, sum);
    }
    else//不是有效路径 +0
    {
        return dfs(root->left, targetSum, sum) + dfs(root->right, targetSum, sum);
    }
}

/*
*int pathSum(struct TreeNode* root, int targetSum)
int pathSum：寻找二叉树中路径个数
struct TreeNode* root：树节点
int targetSum：路径大小
返回值：二叉树的路径数
*/
int pathSum(struct TreeNode* root, int targetSum){
    if(root == NULL)
    {
        return 0;
    }
    //返回当前节点的路径数 + 左子树的路径数 + 右子树的路径数
    return dfs(root, targetSum, (long)0) + pathSum(root->left, targetSum) + pathSum(root->right, targetSum);
}

