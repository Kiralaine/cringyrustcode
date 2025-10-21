// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = match (&l1, &l2) {
                (Some(a), Some(b)) => a.val + b.val + carry,
                (Some(a), None) => a.val + carry,
                (None, Some(b)) => b.val + carry,
                (None, None) => carry,
            };

            carry = sum / 10;
            let node = Box::new(ListNode::new(sum % 10));
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();

            if let Some(a) = l1 {
                l1 = a.next;
            } else {
                l1 = None;
            }

            if let Some(b) = l2 {
                l2 = b.next;
            } else {
                l2 = None;
            }
        }

        dummy.next
    }
}
