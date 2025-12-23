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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0  {
            let v1: i32 = p1.map(|n| n.val).unwrap_or(0);
            let v2 = p2.map(|n| n.val).unwrap_or(0);
            let sum = v1 + v2 + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
            p1 = p1.and_then(|n| n.next.as_ref());
            p2 = p2.and_then(|n| n.next.as_ref());
        }
        dummy.next
    }
}
fn main() {
    println!("Hello, world!");
}
