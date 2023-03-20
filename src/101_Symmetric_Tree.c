// 101_Symmetric_Tree
// https://leetcode.cn/problems/symmetric-tree/

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

bool isSameTree(struct TreeNode* p, struct TreeNode* q){
    if (p == NULL && q == NULL) {
        return true;
    }
    if (p != NULL && q == NULL) {
        return false;
    }
    if (p == NULL && q != NULL) {
        return false;
    }
    if (p->val == q->val) {
        return isSameTree(p->left, q->right) && isSameTree(p->right, q->left);
    } else {
        return false;
    }
}

bool isSymmetric(struct TreeNode* root){
    return isSameTree(root->left, root->right);
}



/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
bool check(struct TreeNode* p,struct TreeNode* q){
    if(!p && !q) return true;
    if(!p || !q) return false;
    return p->val==q->val && check(p->left,q->right) && check(p->right,q->left);
}
bool isSymmetric(struct TreeNode* root){
    return check(root,root);
}

// 「方法一」中我们用递归的方法实现了对称性的判断，
// 那么如何用迭代的方法实现呢？
// 首先我们引入一个队列，这是把递归程序改写成迭代程序的常用方法。
// 初始化时我们把根节点入队两次。
// 每次提取两个结点并比较它们的值（队列中每两个连续的结点应该是相等的，而且它们的子树互为镜像），
// 然后将两个结点的左右子结点按相反的顺序插入队列中。
// 当队列为空时，或者我们检测到树不对称（即从队列中取出两个不相等的连续结点）时，该算法结束。

// def bfs(node):
//     if node is None:
//         return
    
//     queue = []
//     queue.append(node)
    
//     while len(queue) > 0:
//         current_node = queue.pop(0)
//         print(current_node.value)
//         if current_node.left is not None:
//             queue.append(current_node.left)
//         if current_node.right is not None:
//             queue.append(current_node.right)
