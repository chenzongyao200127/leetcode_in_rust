// 143_Reorder_List
// https://leetcode.cn/problems/reorder-list/


// Definition for singly-linked list.
class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}
 

// class Solution {
//     public void reorderList(ListNode head) {
//         if (head == null || head.next == null || head.next.next == null) {
//             return;
//         }
        
//         ListNode fast = head;
//         ListNode slow = head;

//         while (fast != null && fast.next != null) {
//             fast = fast.next.next;
//             slow = slow.next;
//         }

//         ListNode head2 = slow.next;
//         slow.next = null;

//         ListNode rev_head2 = reverseList(head2);

//         System.out.println(rev_head2);

//         ListNode c1 = head;
//         ListNode n1 = head.next;
//         ListNode c2 = rev_head2;
//         ListNode n2 = rev_head2.next;

//         while (n1 != null && c2 != null ) {
//             c1.next = c2;
//             c2.next = n1;
//             c1 = n1;
//             if (n1 != null) {
//                 n1 = n1.next;
//             }
//             c2 = n2;
//             if (n2 != null) {
//                 n2 = n2.next;
//             }
//         }

//         return;
//     }

//     public ListNode reverseList(ListNode head) {
//         if (head == null || head.next == null) {
//             return head;
//         }
//         ListNode pre = null;
//         ListNode cur = head;
//         ListNode nex = head.next;

//         while (cur != null) {
//             cur.next = pre;
//             pre = cur;
//             cur = nex;
//             if (nex != null) {
//                 nex = nex.next;
//             }
//         }

//         return pre;
//     }
// }


class Solution {
    public void reorderList(ListNode head) {
        if (head == null || head.next == null || head.next.next == null) {
            return;
        }

        ListNode fast = head, slow = head;
        while (fast != null && fast.next != null) {
            fast = fast.next.next;
            slow = slow.next;
        }

        ListNode head2 = slow.next;
        slow.next = null;
        ListNode revHead2 = reverseList(head2);

        ListNode c1 = head, c2 = revHead2;
        while (c1 != null && c2 != null) {
            ListNode n1 = c1.next, n2 = c2.next;
            c1.next = c2;
            c2.next = n1;
            c1 = n1;
            c2 = n2;
        }
    }

    public ListNode reverseList(ListNode head) {
        ListNode pre = null, cur = head;
        while (cur != null) {
            ListNode nex = cur.next;
            cur.next = pre;
            pre = cur;
            cur = nex;
        }
        return pre;
    }
}
