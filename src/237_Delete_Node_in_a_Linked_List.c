// 237. Delete Node in a Linked List
// https://leetcode.cn/problems/delete-node-in-a-linked-list/

// 4 ms 94.19%
// 6.3 MB 34.72%

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode
{
    int val;
    struct ListNode *next;
};

// 这道题细思极恐：如何让自己在世界上消失，但又不死？ —— 将自己完全变成另一个人，再杀了那个人就行了。
void deleteNode(struct ListNode *node)
{
    struct ListNode *tmp = node->next;

    node->val = tmp->val;
    node->next = tmp->next;
    free(tmp);
}