// https://leetcode.cn/problems/rotate-list/
// 4 ms 71.43%
// 6 MB 59.61%

#include <stdio.h>

// Definition for singly-linked list.
struct ListNode
{
    int val;
    struct ListNode *next;
};

// struct ListNode* rotateRight(struct ListNode* head, int k){
//     struct ListNode dummy;
//     struct ListNode *tmp = head;

//     int len = get_length(head);
//     if (len == 0) {
//         return head;
//     }
//     k = k % len;
//     for (int i=0; i < len-k; i++) {
//         tmp = tmp->next;
//     }
//     dummy.next = tmp;
//     while (tmp) {
//         tmp = tmp->next;
//     }
//     tmp->next = head;
//     for (int i=0; i<len-k-1; i++) {
//         head = head->next;
//     }
//     head->next = NULL;

//     return dummy.next;
// }

struct ListNode *rotateRight(struct ListNode *head, int k)
{
    if (!head || !k)
        return head;
    int len = 0;
    struct ListNode *tail;
    for (struct ListNode *pre = head; pre; pre = pre->next)
    {
        tail = pre;
        ++len;
    }
    k %= len;
    struct ListNode *pre = head;
    for (int i = 0; i < len - k - 1; i++)
    {
        pre = pre->next;
    } // 找到链表的第n-k个节点
    tail->next = head;
    head = pre->next;
    pre->next = NULL;

    return head;
}
