// https://leetcode.cn/problems/reverse-linked-list-ii/

#include <stdio.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

// void reverseLinkedList(struct ListNode *head) {
//     // 也可以使用递归反转一个链表
//     struct ListNode *pre = NULL;
//     struct ListNode *cur = head;

//     while (cur != NULL) {
//         struct ListNode *next = cur->next;
//         cur->next = pre;
//         pre = cur;
//         cur = next;
//     }
// }

struct ListNode *reverseLinkedList(struct ListNode *head)
{
    struct ListNode *pre = NULL;
    struct ListNode *left = head;
    struct ListNode *right = NULL;

    while (left)
    {
        right = left->next;
        left->next = pre;
        pre = left;
        left = right;
    }

    return pre;
}

struct ListNode *reverseBetween(struct ListNode *head, int left, int right)
{
    // 因为头节点有可能发生变化，使用虚拟头节点可以避免复杂的分类讨论
    struct ListNode *dummyNode = malloc(sizeof(struct ListNode));
    dummyNode->val = -1;
    dummyNode->next = head;

    struct ListNode *pre = dummyNode;
    // 第 1 步：从虚拟头节点走 left - 1 步，来到 left 节点的前一个节点
    // 建议写在 for 循环里，语义清晰
    for (int i = 0; i < left - 1; i++)
    {
        pre = pre->next;
    }

    // 第 2 步：从 pre 再走 right - left + 1 步，来到 right 节点
    struct ListNode *rightNode = pre;
    for (int i = 0; i < right - left + 1; i++)
    {
        rightNode = rightNode->next;
    }

    // 第 3 步：切断出一个子链表（截取链表）
    struct ListNode *leftNode = pre->next;
    struct ListNode *succ = rightNode->next;

    // 注意：切断链接
    pre->next = NULL;
    rightNode->next = NULL;

    // 第 4 步：同第 206 题，反转链表的子区间
    reverseLinkedList(leftNode);

    // 第 5 步：接回到原来的链表中
    pre->next = rightNode;
    leftNode->next = succ;

    return dummyNode->next;
}
