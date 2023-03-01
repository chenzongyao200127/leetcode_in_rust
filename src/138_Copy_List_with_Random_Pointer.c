// 138. Copy List with Random Pointer
// https://leetcode.cn/problems/copy-list-with-random-pointer/

#include<stdio.h>

//  Definition for a Node.
struct Node {
    int val;
    struct Node *next;
    struct Node *random;
};

// int getLength(struct Node* node) {
//     int length = 0;
//     while (node != NULL) {
//         node = node->next;
//         length++;
//     }

//     return length;
// }


// 方法二：迭代 + 节点拆分
struct Node* copyRandomList(struct Node* head) {
    if (head == NULL) {
        return NULL;
    }

    for (struct Node* node = head; node != NULL; node = node->next->next) {
        struct Node* nodeNew = malloc(sizeof(struct Node));
        nodeNew->val = node->val;
        nodeNew->next = node->next;
        node->next = nodeNew;
    }

    for (struct Node* node = head; node != NULL; node = node->next->next) {
        struct Node* nodeNew = node->next;
        nodeNew->random = (node->random != NULL) ? node->random->next : NULL;
    }

    struct Node* headNew = head->next;
    for (struct Node* node = head; node != NULL; node = node->next) {
        struct Node* nodeNew = node->next;
        node->next = node->next->next;
        nodeNew->next = (nodeNew->next != NULL) ? nodeNew->next->next : NULL;
    }

    return headNew;
}

/**
 * Definition for a Node.
 * struct Node {
 *     int val;
 *     struct Node *next;
 *     struct Node *random;
 * };
 */

struct Node* copyRandomList(struct Node* head) {
	struct Node* cur=head;
    //将原结点复制并插入原结点之后
    while(cur)
    {
        struct Node* copy=(struct Node*)malloc(sizeof(struct Node));
        copy->val=cur->val;
        copy->next=cur->next;
        cur->next=copy;
        cur=copy->next;
    }

    //对copy结点的random指针进行处理
    cur=head;
    while(cur)
    {
        struct Node* copy=cur->next;
        if(cur->random==NULL)
            copy->random=NULL;
        else
        {
            copy->random=cur->random->next;
        }
        cur=copy->next;
    }

    //将复制的结点重新连接并且恢复原链表
    struct Node* copyhead=NULL;
    struct Node* copytail=NULL;
    cur=head;
    while(cur)
    {
        struct Node* copy=cur->next;
        struct Node* next=copy->next;
        if(copytail==NULL)
            copyhead=copytail=copy;
        else
        {
            copytail->next=copy;
            copytail=copy;
        }

        cur->next=next;
        cur=next;
    }
    return copyhead;
}


// class Solution {
// public:
//     unordered_map<Node*, Node*> cachedNode;

//     Node* copyRandomList(Node* head) {
//         if (head == nullptr) {
//             return nullptr;
//         }
//         if (!cachedNode.count(head)) {
//             Node* headNew = new Node(head->val);
//             cachedNode[head] = headNew;
//             headNew->next = copyRandomList(head->next);
//             headNew->random = copyRandomList(head->random);
//         }
//         return cachedNode[head];
//     }
// };
