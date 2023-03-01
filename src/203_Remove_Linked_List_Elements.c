// 203. Remove Linked List Elements
// https://leetcode.cn/problems/remove-linked-list-elements/


// Definition for singly-linked list.
#include<stdio.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode* removeElements(struct ListNode* head, int val){
    struct ListNode dummy;
    struct ListNode *tmp;
    dummy.next = head;
    struct ListNode *prev = &dummy; 

    while (prev->next != NULL) {
        if (prev->next->val == val) {
            tmp = prev->next;
            prev->next = prev->next->next;
            free(tmp);
        } else {
            prev = prev->next;
        }
    }

    return dummy.next;
}

// struct ListNode* removeElements(struct ListNode* head, int val){
//     struct ListNode dummy;
//     struct ListNode *tmp;
//     dummy.next = head;
//     head = &dummy;

//     while (head->next) {
//         if (head->next->val == val) {
//             tmp = head->next;
//             head->next = head->next->next;
//             free(tmp);
//         } else {
//             head = head->next;
//         }
//     }

//     return dummy.next;
// }


struct ListNode* removeElements(struct ListNode* head, int val){
    if(head==NULL)return head;
    struct ListNode* ptr=(struct ListNode* )malloc(sizeof(struct ListNode));
    ptr->next=head;
    struct ListNode*slow=ptr,*fast=ptr->next;
    while(fast!=NULL){
        if(fast->val==val){
            slow->next=fast->next;
            free(fast);
            fast=slow->next;
        }else{
            slow=slow->next;fast=fast->next;
        }
    }
    return ptr->next;
}


struct ListNode* removeElements(struct ListNode* head, int val){
    struct ListNode dummy;
    struct ListNode *tmp;
    dummy.next = head;
    head = &dummy;

    while (head->next) {
        if (head->next->val == val) {
            tmp = head->next;
            head->next = head->next->next;
            free(tmp);
        } else {
            head = head->next;
        }
    }

    return dummy.next;
}

struct ListNode* removeElements(struct ListNode* head, int val){
    while (head!=NULL&&head->val==val) {
        head=head->next;
    }
    if (head==NULL) {
        return head;
    }

    struct ListNode* prev=head;

    //确保当前结点后还有结点
    while (prev->next!=NULL) {
        if (prev->next->val==val) {
            prev->next=prev->next->next;
        } else {
            prev=prev->next;
        }
    }

    return head;
}