// 19. Remove Nth Node From End of List
// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/

// 0ms 100%
// 5.6 MB 67.44%

#include<stdio.h>

/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    struct ListNode *next;
};


struct ListNode* removeNthFromEnd(struct ListNode* head, int n){
    struct ListNode dummy;
    dummy.next = head;
    struct ListNode *prev = &dummy;
    struct ListNode *tmp;
    int len = get_length(head);

    prev = &dummy;
    for (int i = 0; i < len-n ; i++) {
        prev = prev->next;
    }
    tmp = prev->next;
    prev->next = prev->next->next;
    free(tmp);

    return dummy.next;
}

int get_length(struct ListNode* head) {
    int length = 0;
    while (head) {
        ++length;
        head = head->next;
    }
    return length;
}

struct ListNode* removeNthFromEnd(struct ListNode* head, int n) {
    struct ListNode* dummy = malloc(sizeof(struct ListNode));
    dummy->val = 0, dummy->next = head;
    struct ListNode* first = head;
    struct ListNode* second = dummy;
    
    for (int i = 0; i < n; ++i) {
        first = first->next;
    }
    while (first) {
        first = first->next;
        second = second->next;
    }
    second->next = second->next->next;
    struct ListNode* ans = dummy->next;
    free(dummy);
    return ans;
}
// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/remove-nth-node-from-end-of-list/solution/shan-chu-lian-biao-de-dao-shu-di-nge-jie-dian-b-61/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


// class Solution {
// public:
//     int cur=0;
//     ListNode* removeNthFromEnd(ListNode* head, int n) {
//        if(!head) return NULL;
//        head->next = removeNthFromEnd(head->next,n);
//        cur++;
//        if(n==cur) return head->next;
//        return head;
//     }
// };

struct ListNode* removeNthFromEnd(struct ListNode* head, int n){
    struct ListNode dummy;
    dummy.next = head;
    struct ListNode *prev = &dummy;
    struct ListNode *tmp;
    int len = 0;

    while (head != NULL) {
        head = head->next;
        len += 1;
    }

    prev = &dummy;
    for (int i = 0; i < len-n ; i++) {
        prev = prev->next;
    }
    tmp = prev->next;
    prev->next = prev->next->next;
    free(tmp);

    return dummy.next;
}