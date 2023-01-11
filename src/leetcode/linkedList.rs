
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>>{
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node))
    }
    current
}

pub fn l19_remove_nth_node1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode {
        val: 0,
        next: head
    }));
    let mut cur = &mut dummy;
    let mut length = 0;
    while let Some(node) = cur.as_mut() {
        cur = &mut node.next;
        if let Some(_node) = cur {
            length += 1;
        }
    }

    let mut new_cur = dummy.as_mut();
    let idx = length - n;

    for _ in 0..idx {
        new_cur = new_cur.unwrap().next.as_mut();
    }

    let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
    new_cur.as_mut().unwrap().next = next;
    dummy.unwrap().next
}
pub fn l19_remove_nth_node2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut slow_p = &mut dummy;
    let mut fast_p = &mut slow_p.clone();

    for _ in 1..=n + 1 {
        fast_p = &mut fast_p.as_mut().unwrap().next;
    }

    while fast_p.is_some() {
        fast_p = &mut fast_p.as_mut().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
    }

    let next = &slow_p.as_mut().unwrap().next.as_mut().unwrap().next;
    slow_p.as_mut().unwrap().next = next.clone();

    dummy.unwrap().next
}

pub fn l21_merge_two_sorted_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
    match (l1,l2) {
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = l21_merge_two_sorted_lists(n, Some(node2));
                Some(node1)
            } else {
                let n = node2.next.take();
                node2.next = l21_merge_two_sorted_lists(Some(node1),n);
                Some(node2)
            }
        }
        _ => None
    }
}



pub fn l206_reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut current_node) = curr.take() {
        let next_temp = current_node.next.take();
        current_node.next = prev.take();
        prev = Some(current_node);
        curr = next_temp;
    }
    prev
}