#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res: Option<Box<ListNode>> = None;
    let mut current = &mut res;
    let mut c1 = l1;
    let mut c2 = l2;
    let mut carry = 0;

    while c1.is_some() || c2.is_some() || carry != 0 {
        let sum = match (c1.clone(), c2.clone()) {
            (Some(node1), Some(node2)) => {
                c1 = node1.next;
                c2 = node2.next;
                node1.val + node2.val + carry
            }
            (Some(node1), None) => {
                c1 = node1.next;
                node1.val + carry
            }
            (None, Some(node2)) => {
                c2 = node2.next;
                node2.val + carry
            }
            (None, None) => carry,
        };

        carry = sum / 10;
        let new_node = ListNode::new(sum % 10);
        *current = Some(Box::new(new_node));
        current = &mut current.as_mut().unwrap().next;
    }

    res
}

fn main() {
    let mut n0 = ListNode::new(0);
    n0.next = Some(Box::new(ListNode::new(1)));
    n0.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

    let mut m0 = ListNode::new(0);
    m0.next = Some(Box::new(ListNode::new(1)));
    m0.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

    println!(
        "{:?}",
        add_two_numbers(Some(Box::new(n0)), Some(Box::new(m0)))
    );
}
