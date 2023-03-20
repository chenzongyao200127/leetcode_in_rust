// 226_Invert_Binary_Tree
// https://leetcode.cn/problems/invert-binary-tree/

#include<stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

struct TreeNode* invertTree(struct TreeNode* root){
    struct TreeNode *temp;

    if (root == NULL) {
        return NULL;
    }

    temp = root->left;
    root->left = root->right;
    root->right = temp;

    invertTree(root->left);
    invertTree(root->right);

    return root;
}


struct TreeNode* invertTree(struct TreeNode* root) {
    if (root == NULL) {
        return NULL;
    }

    struct TreeNode* left = invertTree(root->left);
    struct TreeNode* right = invertTree(root->right);
    
    root->left = right;
    root->right = left;

    return root;
}