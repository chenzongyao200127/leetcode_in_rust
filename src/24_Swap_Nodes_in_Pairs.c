// 24. Swap Nodes in Pairs
// https://leetcode.cn/problems/swap-nodes-in-pairs/

// 4 ms 39.6%
// 5.6 MB 78.05%

#include <stdio.h>

// Definition for singly-linked list.
struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *swapPairs(struct ListNode *head)
{
    struct ListNode dummy;
    dummy.next = head;
    struct ListNode *prev = &dummy;

    while (prev->next && prev->next->next)
    {
        struct ListNode *left = prev->next;
        struct ListNode *right = prev->next->next;
        prev->next = right;
        left->next = right->next;
        right->next = left;
        prev = left;
    }

    return dummy.next;
}

struct ListNode *swapPairs(struct ListNode *head)
{
    if (head == NULL || head->next == NULL)
    {
        return head;
    }
    struct ListNode *newHead = head->next;
    head->next = swapPairs(newHead->next);
    newHead->next = head;
    return newHead;
}
// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/swap-nodes-in-pairs/solution/liang-liang-jiao-huan-lian-biao-zhong-de-jie-di-91/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。