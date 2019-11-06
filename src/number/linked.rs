use std::{mem, ptr};
use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct LinkedNumber {
    val: usize,
    head: Node,
}

#[derive(Debug)]
pub struct Node {
    val: usize,
    next: Option<Box<Node>>,
}

impl LinkedNumber {
    pub fn new(val: usize) -> Self {
        let head = Node::new(val);
        LinkedNumber { val, head }
    }

    pub fn plus_one(&mut self) {
        self.val += 1;
        self.head.plus_one();
    }

    pub fn value(&self) -> &usize {
        &self.val
    }

    pub fn node(&self) -> &Node {
        &self.head
    }

    pub fn calculate_value(&self) -> usize {
        let mut sum = self.head.val;
        let mut pow = 10;
        let mut next = self.head.next.as_ref();
        while let Some(node) = next {
            sum += node.val * pow;
            next = node.next.as_ref();
            pow *= 10;
        }
        sum
    }

    pub fn reverse(&mut self) {
        if let Some(second) = self.head.next.take() {
            //let head = mem::replace(&mut self.head, Node::default());
            //let raw_head: *const _ = ;
            unsafe {
                let head = ptr::read(&self.head);
                let mut prev = Some(Box::new(head));
                let mut curr = second;
                while let Some(next) = curr.next.take() {
                    curr.next = prev;
                    prev = Some(curr);
                    curr = next;
                }
                curr.next = prev;
                self.head = *curr;
            }
            self.val = self.calculate_value();
        }
    }

    pub fn reverse_at(&mut self, index: usize) {
        if let Some(second) = self.head.next.take() {
            //let raw_head: *const _ = &self.head;
            unsafe {
                let head = ptr::read(&self.head);
                let boxed_head = Box::new(head);
                let middle_tail: *const Node = &*boxed_head;
                let mut prev = Some(boxed_head);
                let mut curr = second;
                let mut middle_head = None;
                let mut i = 1;
                loop {
                    let next = curr.next.take();
                    if i == index {
                        middle_head = prev;
                    } else {
                        curr.next = prev;
                    }
                    match next {
                        Some(next) => {
                            prev = Some(curr);
                            curr = next;
                            i += 1;
                        }
                        None => break,
                    }
                }
                match middle_head {
                    Some(head) => {
                        let ptr = middle_tail as *mut Node;
                        (*ptr).next = Some(curr);
                        self.head = *head
                    }
                    None => self.head = *curr,
                }
            }
            self.val = self.calculate_value();
        }
    }
}

impl Node {
    pub fn new(i: usize) -> Self {
        let j = i / 10;
        if j != 0 {
            let val = i - j * 10;
            let mut head = Node { val, next: None };
            Node::new0(j, &mut head);
            head
        } else {
            Node {
                val: i,
                next: None,
            }
        }
    }

    fn new0(i: usize, prev: &mut Node) {
        let j = i / 10;
        if j != 0 {
            let val = i - j * 10;
            let node = Box::new(Node { val, next: None });
            prev.next = Some(node);
            if let Some(ref mut next) = prev.next {
                Node::new0(j, next);
            }
        } else {
            prev.next.replace(Box::new(Node { val: i, next: None }));
        }
    }

    fn plus_one(&mut self) {
        let i = self.val + 1;
        if i == 10 {
            self.val = 0;
            match self.next {
                Some(ref mut next) => Node::plus_one(next.borrow_mut()),
                None => self.next = Some(Box::new(Node { val: 1, next: None })),
            }
        } else {
            self.val = i;
        }
    }

    pub fn value(&self) -> &usize {
        &self.val
    }

    pub fn next(&self) -> Option<&Node> {
        self.next.as_ref().map(|node| &**node)
    }
}

impl Drop for LinkedNumber {
    fn drop(&mut self) {
        let mut curr_node = mem::replace(&mut self.head.next, None);
        while let Some(mut node) = curr_node {
            curr_node = mem::replace(&mut node.next, None)
        }
    }
}

/*
impl Default for Node {
    fn default() -> Self {
        Node {
            val: 0,
            next: None,
        }
    }
}*/
