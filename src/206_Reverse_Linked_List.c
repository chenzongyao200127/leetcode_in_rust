// https://leetcode.cn/problems/reverse-linked-list/

#include <stdio.h>

// Definition for singly-linked list.
struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *reverseList(struct ListNode *head)
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

struct ListNode *reverseList(struct ListNode *head)
{
    if (!head || !head->next)
    {
        return head;
    }

    struct ListNode *others = reverseList(head->next);
    head->next->next = head;
    head->next = NULL;

    return others;
}