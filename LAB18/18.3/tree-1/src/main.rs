use std::cell::RefCell;
use std::rc::Rc;

// Define a node in the binary tree
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// Function to create a new tree node
fn new_node(value: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        value,
        left: None,
        right: None,
    }))
}

fn main() {
    // Create the root node
    let root = new_node(1);

    // Create left and right children
    let left_child = new_node(2);
    let right_child = new_node(3);

    // Attach children to the root
    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    // Traverse and print the tree in order
    print_tree(&root);
}

// Function to traverse and print the tree in-order
fn print_tree(node: &Rc<RefCell<TreeNode>>) {
    if let Some(ref left) = node.borrow().left {
        print_tree(left);
    }
    println!("{}", node.borrow().value);
    if let Some(ref right) = node.borrow().right {
        print_tree(right);
    }
}
