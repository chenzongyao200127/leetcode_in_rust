// // https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/

// // Definition for a Node.
// class Node {
//     public int val;
//     public Node prev;
//     public Node next;
//     public Node child;
// };



// class Solution {
//     public Node flatten(Node head) {
//         dfs(head);
//         return head;
//     }
//     Node dfs(Node head) {
//         Node last = head;
//         while (head != null) {
//             if (head.child == null) {
//                 last = head;
//                 head = head.next;
//             } else {
//                 Node tmp = head.next;
//                 Node childLast = dfs(head.child);
//                 head.next = head.child;
//                 head.child.prev = head;
//                 head.child = null;
//                 if (childLast != null) childLast.next = tmp;
//                 if (tmp != null) tmp.prev = childLast;
//                 last = head;
//                 head = childLast;
//             }
//         }
//         return last;
//     }
// }

// 作者：AC_OIer
// 链接：https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/solution/gong-shui-san-xie-yi-ti-shuang-jie-di-gu-9wfz/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


// class Solution {
//     public Node flatten(Node head) {
//         Node dummy = new Node(0);
//         dummy.next = head;

//         while (head != null) {
//             if (head.child == null) {
//                 head = head.next;
//             } else {
//                 Node tmp = head.next;
//                 Node child_head = flatten(head.child);
//                 head.next = child_head;
//                 child_head.prev = head;
//                 head.child = null;
//                 while (head.next != null) { head = head.next; }
//                 head.next = tmp;
//                 if (tmp != null) { tmp.prev = head; }
//                 head = tmp;
//             }
//         }

//         return dummy.next;
//     }
// }

// class Solution {
//     public Node flatten(Node head) {
//         Node dummy = new Node(0);
//         dummy.next = head;

//         for (; head != null; head = head.next) {
//             if (head.child != null) {
//                 Node tmp = head.next;
//                 Node child = head.child;
//                 head.next = child;
//                 child.prev = head;
//                 head.child = null;
//                 Node last = head;
//                 while (last.next != null) last = last.next;
//                 last.next = tmp;
//                 if (tmp != null) tmp.prev = last;
//             }
//         }
//         return dummy.next;
//     }
// }

// // 作者：AC_OIer
// // 链接：https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/solution/gong-shui-san-xie-yi-ti-shuang-jie-di-gu-9wfz/
// // 来源：力扣（LeetCode）
// // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。