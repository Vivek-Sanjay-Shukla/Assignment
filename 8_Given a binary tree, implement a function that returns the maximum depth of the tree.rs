use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    data: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}


impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: val,
            left: None,
            right: None,
        }))
    }
}

fn solve(node: Option<Rc<RefCell<Node>>>) -> i32 {
    match node {
        Some(n) => {
            let n = n.borrow();
            let l1 = solve(n.left.clone());
            let r1 = solve(n.right.clone());
            if l1 > r1 {
                l1 + 1
            } else {
                r1 + 1
            }
        }
        None => 0,
    }
}

fn main() {
    let root = Node::new(1);
    let left = Node::new(2);
    let right = Node::new(3);
    let left_left = Node::new(4);
    let left_right = Node::new(5);

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    left.borrow_mut().left = Some(left_left.clone());
    left.borrow_mut().right = Some(left_right.clone());

    println!("Maximum depth of tree is: {}", solve(Some(root)));
}
