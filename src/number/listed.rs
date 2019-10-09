use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct LinkedNumber {
    pub val: usize,
    pub head: Node,
}

#[derive(Debug)]
pub struct Node {
    pub val: usize,
    pub next: Option<Box<Node>>,
}

impl LinkedNumber {
    pub fn new(val: usize) -> LinkedNumber {
        let head = Node::new(val);
        LinkedNumber { val, head }
    }

    pub fn plus_one(&mut self) {
        self.val += 1;
        self.head.plus_one();
    }
}

impl Node {
    fn new(i: usize) -> Node {
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
}