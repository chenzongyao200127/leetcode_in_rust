// https://leetcode.cn/problems/reverse-nodes-in-k-group/

// 8 ms 69.75%
// 6.8 ms 77.02%

#include<stdio.h>

// Definition for singly-linked list.
struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode* reverseList(struct ListNode* head) {
    struct ListNode *pre = NULL;
    struct ListNode *cur = head;
    struct ListNode *tmp = NULL;

    while (cur) {
        tmp = cur->next;
        cur->next = pre;
        pre = cur;
        cur = tmp;
    }

    return pre;
}


struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode dummy;
    dummy.next = head;
    int len = get_length(dummy.next);
    int times = len / k;

    struct ListNode *before = &dummy;
    for (int i=0; i<times; i++) {
        struct ListNode *left_node = before->next;
        struct ListNode *right_node = before;
        for (int j=0; j<k; j++) {
            right_node = right_node->next;
        }
        struct ListNode *succ = right_node->next;

        before->next = NULL;
        right_node->next = NULL;

        reverseList(left_node);

        before->next = right_node;
        left_node->next = succ;

        for (int m=0; m<k; m++) {
            before = before->next;
        }
    }

    return dummy.next;
}


int get_length(struct ListNode* head) {
    int len = 0;
    while (head) {
        len++ ;
        head = head->next;
    }

    return len;
}