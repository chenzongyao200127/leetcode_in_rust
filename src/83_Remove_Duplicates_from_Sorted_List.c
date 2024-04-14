// 83_Remove_Duplicates_from_Sorted_List
// https://leetcode.cn/problems/remove-duplicates-from-sorted-list/

/**
 * Definition for singly-linked list.
 */
#include <stdio.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *deleteDuplicates(struct ListNode *head)
{
    struct ListNode *cur = head;
    if (head == NULL)
    {
        return head;
    }
    // 若 head == NULL 则会访问空指针
    struct ListNode *idx = head->next;
    if (idx == NULL)
    {
        return head;
    }
    // 逻辑是正确的
    while (idx != NULL)
    {
        if (idx->val != cur->val)
        {
            cur->next = idx;
            idx = idx->next;
            cur = cur->next;
        }
        else
        {
            idx = idx->next;
        }
    }
    // 添加尾部NULL
    if (cur != NULL)
    {
        cur->next = NULL;
    }

    return head;
}

struct ListNode *deleteDuplicates(struct ListNode *head)
{
    struct ListNode *p = head, *pr = p;
    while (head != NULL)
    {
        if (p->val == head->val)
        {
            head = head->next;
        }
        else
        {
            p->next = head;
            p = p->next;
            head = head->next;
        }
    }
    if (p != NULL)
        p->next = NULL;
    return pr;
}