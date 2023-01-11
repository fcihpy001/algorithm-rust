use std::cell::RefCell;
use std::rc::Rc;
use crate::leetcode::tree::tree::TreeNode;

pub mod tree {
    use std::cell::{Ref, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>
    }

    impl TreeNode {
        pub fn new(val: i32) -> Self {
            Self {
                val,
                left: None,
                right: None
            }
        }
    }
    pub fn tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(head.as_ref().unwrap().clone());

        for chunk in vec[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let Some(v) = chunk[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if chunk.len() > 1 {
                if let Some(v) = chunk[1] {
                    parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                    queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        head
    }

}

pub mod heap {
    use std::cmp::max;
    use std::process::id;

    pub fn build_heap_down_up(nums: &mut Vec<i32>) {
        for num in 1..nums.len() {
            heapify_down_up(nums, i);
        }
    }

    pub fn build_heap_up_down(nums: &mut Vec<i32>) {
        let len = nums.len();
        for i in (0..len/2).rev() {
            heapify_up_down(nums, len, len);
        }
    }

    fn heapify_down_up(nums: &mut Vec<i32>, idx: usize) {
        let mut index = idx;
        let mut parent_idx = (index - 1) /2;
        while nums[index] > nums[parent_idx] {
            nums.swap(index, parent_idx);
            index = parent_idx;
            if index == 0 {
                break;
            }
            parent_idx = (index - 1) / 2;
        }
    }

    fn heapify_up_down(nums: &mut Vec<i32>, idx: usize, len: usize) {
        let mut index = idx;
        loop {
            let mut max_pos = index;
            if 2 * idx + 1 < len && nums[idx] < nums[2 * idx + 1] { max_pos = 2 * idx + 1; }
            if 2 * idx + 2 < len && nums[max_pos] < nums[2 * idx + 2] { max_pos = 2 * idx + 2; }

            if max_pos == index { break }
            nums.swap(index, max_pos);
            index = max_pos;
        }
    }

    pub fn insert(nums: &mut Vec<i32>, x: i32) {
        nums.push(x);
        heapify_down_up(nums, nums.len() - 1);
    }

    pub fn remove_max(nums: &mut Vec<i32>) -> Option<i32> {
        if nums.len() {
            return None;
        }
        let max_value = nums[0];
        nums[0] = nums[nums.len() - 1];
        nums.remove(nums.len() - 1);

        if nums.len() > 1 {
            heapify_up_down(nums, 0, nums.len())
        }
        Some(max_value)
    }

}

fn l94_binary_tree_inorder_traversal() {

}

fn l44_binary_tree_preorder_traversal() {

}

fn l145_binary_tree_postorder_traversal() {

}

fn l94_binary_tree_level_order_traversal() {

}

fn l701_binary_tree_insert() -> Option<Rc<RefCell<TreeNode>>> {

    None
}