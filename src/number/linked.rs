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
    pub fn new(i: usize) -> Node {
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

    pub fn _reverse(self) -> Box<Node> {
        let mut prev = None;
        let mut curr = Box::new(self);
        loop {
            let next = curr.next.take();
            curr.next = prev;
            match next {
                Some(next_node) => {
                    prev = Some(curr);
                    curr = next_node;
                }
                None => return curr,
            }
        }
    }

    pub fn _reverse_at(self, k: usize) -> Box<Node> {
        //let v = self.val as f64;
        //let length = (v.log10() as usize) + 1;
        //assert!(length > k, "reverse position can't be bigger than node length");

        /*let mut first_node = Box::new(self);
        match first_node.next.take() {
            Some(mut second_node) => {
                let mut first_node = Some(first_node);
                let third = second_node.next.take();
                second_node.next = first_node;
                let first_part_tail = &mut second_node.next;
                match third {
                    Some(third_node) => {
                        let mut prev = Some(second_node);
                        let mut curr = third_node;
                        let mut i: usize = 2;
                        while i <= k {
                            let next = curr.next.take();
                            curr.next = prev;
                            match next {
                                Some(next_node) => {
                                    prev = Some(curr);
                                    curr = next_node;
                                }
                                None => return curr,
                            }
                            i += 1;
                        }
                        let head = prev.unwrap();
                        prev = None;
                        loop {
                            let next = curr.next.take();
                            curr.next = prev;
                            match next {
                                Some(next_node) => {
                                    prev = Some(curr);
                                    curr = next_node;
                                }
                                None => {
                                    first_part_tail.unwrap().next = Some(curr);
                                    return head;
                                }
                            }
                        }
                    }
                    None => return second_node,
                }
            }
            None => return first_node,
        }*/
        let mut prev = None;
        let mut curr = Box::new(self);

        let first_part_tail = &mut curr;

        let mut i: usize = 0;
        while i <= k {
            let next = curr.next.take();
            curr.next = prev;
            match next {
                Some(next_node) => {
                    prev = Some(curr);
                    curr = next_node;
                }
                None => return curr,
            }
            i += 1;
        }

        let head = prev.unwrap();
        prev = None;
        loop {
            let next = curr.next.take();
            curr.next = prev;
            match next {
                Some(next_node) => {
                    prev = Some(curr);
                    curr = next_node;
                }
                None => {
                    first_part_tail.next = Some(curr);
                    return head;
                }
            }
        }
    }
}