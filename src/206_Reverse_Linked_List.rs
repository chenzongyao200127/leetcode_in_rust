// https://leetcode.cn/problems/reverse-linked-list/

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
}
// 作者：tab-liu
// 链接：https://leetcode.cn/problems/reverse-linked-list/solution/206-fan-zhuan-lian-biao-by-tab-liu-hp7g/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。