// 82. Remove Duplicates from Sorted List II
// https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/description/

#include <iostream>
using namespace std;

// Definition for singly-linked list.
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution
{
public:
    ListNode *deleteDuplicates(ListNode *head)
    {
        if (!head || !head->next)
            return head;

        ListNode *dummy = new ListNode(0); // dummy node to handle edge cases
        dummy->next = head;
        ListNode *prev = dummy; // previous node to the current sequence of duplicates

        while (head)
        {
            // check if the current node is a start of duplicates
            if (head->next && head->val == head->next->val)
            {
                // skip the nodes whose values are equal to head's value
                while (head->next && head->val == head->next->val)
                {
                    head = head->next;
                }
                prev->next = head->next; // unlink the duplicates
            }
            else
            {
                prev = prev->next; // move prev node
            }
            head = head->next; // move head node
        }

        ListNode *newHead = dummy->next;
        delete dummy; // don't forget to delete dummy node to avoid memory leak
        return newHead;
    }
};