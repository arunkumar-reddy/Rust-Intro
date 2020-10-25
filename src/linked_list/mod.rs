pub struct Node {
    value: u32,
    next: Option<Box<Node> >
}

pub struct LinkedList {
    head: Option<Box<Node> >,
}

impl Node {
    pub fn new(value: u32) -> Node {
        Node {
            value: value,
            next: None
        }
    }
}

impl LinkedList{
    pub fn new() -> LinkedList {
        LinkedList {
            head: None
        }
    }

    /*pub fn push_back(&mut self, num: u32) {
        match &mut self.head {
            None => {
                let node = Box::<Node>::new(Node {
                    value: num,
                    next: None
                });
                self.head = Some(node) 
            },
            Some(head) => {
                let mut node = Box::<Node>::new(Node {
                    value: num,
                    next: head.next
                });
                self.head = Some(node);
            }
        }
    }*/

    pub fn print_list(&self) {
        match &self.head {
            None => { println!("Empty Linked List")},
            Some(head) => {
                let mut curr = &head.next;
                loop {
                    match curr {
                        None => break,
                        Some(temp) => {
                            println!("{}", temp.value);
                            curr = &temp.next;
                        }
                    }
                }
            }
        }
    }
}