// https://leetcode.cn/problems/merge-k-sorted-lists/

// 168 ms 45.41%
// 8.3 MB 47.79%
#include <stdio.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *mergeTwoLists(struct ListNode *list1, struct ListNode *list2);

// 顺序合并
// 我们可以想到一种最朴素的方法：用一个变量 ans来维护以及合并的链表，第 i 次循环把第 i 个链表和 ans合并，答案保存到 ans 中。
// O(n*k^2)  O(1)
struct ListNode *mergeKLists(struct ListNode **lists, int listsSize)
{
    struct ListNode *ans = NULL;

    for (int i = 0; i < listsSize; i++)
    {
        ans = mergeTwoLists(ans, lists[i]);
    }

    return ans;
}

struct ListNode *mergeTwoLists(struct ListNode *list1, struct ListNode *list2)
{
    struct ListNode *dummy = (struct ListNode *)malloc(sizeof(struct ListNode));
    struct ListNode *tail = dummy;
    tail->next = NULL;

    while (list1 && list2)
    {
        if (list1->val <= list2->val)
        {
            tail->next = list1;
            list1 = list1->next;
        }
        else
        {
            tail->next = list2;
            list2 = list2->next;
        }
        tail = tail->next;
    }

    tail->next = list1 ? list1 : list2;

    return dummy->next;
}

// 方法二：分治合并
// 方法三：使用优先队列合并