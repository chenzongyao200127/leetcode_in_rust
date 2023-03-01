// https://leetcode.cn/problems/reverse-nodes-in-k-group/

// 0 ms 100%
// 2,2 MB 82.93%

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


// 本质就是某个节点最多只有一个可变引用问题，不然代码里面就没有必要迭代每段翻转后的一组链表找到它的尾结点当下一组的前驱
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut remain = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while remain.is_some() {
            let (new_head, new_remain) = Solution::reverse_one(remain, k);
            remain = new_remain;
            tail.next = new_head;
            while tail.next.as_ref().is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    // 反转一次，返回反转后的head和remain
    // 如果为最后一次不足以反转，remain为None
    fn reverse_one(head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut pre = head.as_ref();
        for _ in 0..k {
            if pre.is_none() {
                return (head, None);
            }
            pre = pre.unwrap().next.as_ref();
        }

        let mut remain = head;
        let mut dummy = ListNode::new(0);
        for _ in 0..k {
            if let Some(mut n) = remain {
                remain = n.next.take();
                n.next = dummy.next.take();
                dummy.next = Some(n);
            }
        }

        (dummy.next, remain)
    }
}
// 作者：psy-core
// 链接：https://leetcode.cn/problems/reverse-nodes-in-k-group/solution/rustlian-biao-cao-zuo-by-psy-core-o6j3/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。