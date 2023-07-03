// 445. Add Two Numbers II
// https://leetcode.cn/problems/add-two-numbers-ii/

#include<cstdio>
#include<cstring>
#include<algorithm>
#include<iostream>
#include<string>
#include<vector>
#include<stack>
#include<bitset>
#include<cstdlib>
#include<cmath>
#include<set>
#include<list>
#include<deque>
#include<map>
#include<queue>
using namespace std;
typedef long long ll;
const double PI = acos(-1.0);
const double eps = 1e-6;
const int INF = 0x3f3f3f3f;

 
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};
 

// 本题的主要难点在于链表中数位的顺序与我们做加法的顺序是相反的，
// 为了逆序处理所有数位，我们可以使用栈：把所有数字压入栈中，再依次取出相加。计算过程中需要注意进位的情况。
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        stack<int> s1, s2;
        while (l1) {
            s1.push(l1 -> val);
            l1 = l1 -> next;
        }
        while (l2) {
            s2.push(l2 -> val);
            l2 = l2 -> next;
        }
        int carry = 0;
        ListNode* ans = nullptr;
        while (!s1.empty() or !s2.empty() or carry != 0) {
            int a = s1.empty() ? 0 : s1.top();
            int b = s2.empty() ? 0 : s2.top();
            if (!s1.empty()) s1.pop();
            if (!s2.empty()) s2.pop();
            int cur = a + b + carry;
            carry = cur / 10;
            cur %= 10;
            auto curnode = new ListNode(cur);
            curnode -> next = ans;
            ans = curnode;
        }
        return ans;
    }
};


class Solution {
    ListNode *reverseList(ListNode *head) {
        if (head == nullptr || head->next == nullptr)
            return head;
        auto new_head = reverseList(head->next);
        head->next->next = head; // 把下一个节点指向自己
        head->next = nullptr; // 断开指向下一个节点的连接，保证最终链表的末尾节点的 next 是空节点
        return new_head;
    }

    // l1 和 l2 为当前遍历的节点，carry 为进位
    ListNode *addTwo(ListNode *l1, ListNode *l2, int carry = 0) {
        if (l1 == nullptr && l2 == nullptr) // 递归边界：l1 和 l2 都是空节点
            return carry ? new ListNode(carry) : nullptr; // 如果进位了，就额外创建一个节点
        if (l1 == nullptr) // 如果 l1 是空的，那么此时 l2 一定不是空节点
            swap(l1, l2); // 交换 l1 与 l2，保证 l1 非空，从而简化代码
        carry += l1->val + (l2 ? l2->val : 0); // 节点值和进位加在一起
        l1->val = carry % 10; // 每个节点保存一个数位
        l1->next = addTwo(l1->next, (l2 ? l2->next : nullptr), carry / 10); // 进位
        return l1;
    }

public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        l1 = reverseList(l1);
        l2 = reverseList(l2); // l1 和 l2 反转后，就变成【2. 两数相加】了
        auto l3 = addTwo(l1, l2);
        return reverseList(l3);
    }
};
