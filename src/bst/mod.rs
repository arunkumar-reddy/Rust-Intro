pub struct BST {
    head: Option<Box<Node> >
}

pub struct Node {
    value: i32,
    left: Option<Box<Node> >,
    right: Option<Box<Node> >
}

impl Node {
    pub fn create(val: i32) -> Box<Node> {
        Box::<Node>::new(Node {
            value: val,
            left: None,
            right: None
        })
    }

    pub fn inorder(&self) {
        match &self.left {
            None => {},
            Some(left) => left.inorder()
        }
        println!("Print node: {}", self.value);
        match &self.right {
            None => {},
            Some(right) => right.inorder()
        }
    }

    pub fn insert(&mut self, val: i32) {
        if self.value > val {
            match &mut self.left {
                None => self.left = Some(Node::create(val)),
                Some(left) => left.insert(val)
            }
        } else if self.value < val {
            match &mut self.right {
                None => self.right = Some(Node::create(val)),
                Some(right) => right.insert(val)
            }
        }
    }

    pub fn search(&self, val: i32) -> Option<&Node> {
        if self.value > val {
            match &self.left {
                None => None,
                Some(left) => left.search(val)
            }
        } else if self.value < val {
            match &self.right {
                None => None,    
                Some(right) => right.search(val)
            } 
        } else {
            Some(self)
        }
    }

    pub fn delete(mut self, val: i32) -> Option<Box<Node> > {
        if self.value > val {
            match self.left {
                None => {},
                Some(left) => {
                    if left.value == val {
                        self.left = left.delete_self();
                    } else {
                        self.left = left.delete(val);
                    }
                }
            }
        } else if self.value < val {
            match self.right {
                None => {},
                Some(right) => {
                    if right.value == val {
                        self.right = right.delete_self();
                    } else {
                        self.right = right.delete(val);
                    }
                }
            }
        }
        Some(Box::new(self))
    }

    fn delete_self(self) -> Option<Box<Node> > {
        match self.left {
            None => {
                match self.right {
                    None => None,
                    Some(right) => Some(right)
                }
            },
            Some(left) => {
                match self.right {
                    None => Some(left),
                    Some(right) => {
                        let new_value = left.inorder_successor();
                        let mut node = Node::create(new_value);
                        node.left = left.delete(new_value);
                        node.right = Some(right);
                        Some(node)
                    }
                }
            }
        }
    }

    fn inorder_successor(&self) -> i32 {
        let mut curr = self;
        loop {
            match &curr.right {
                None => break,
                Some(next) => {
                    curr = next;
                }
            }
        }
        curr.value
    }
}


impl BST {
    pub fn new() -> BST {
        BST {
            head: None
        }
    }

    pub fn insert(&mut self, val: i32) {
        match &mut self.head {
            None => self.head = Some(Node::create(val)),
            Some(head) => head.insert(val)
        }
    }

    pub fn search(&self, val: i32) -> Option<&Node> {
        match &self.head {
            None => {
                println!("BST is empty. Cannot search for: {}", val);
                None
            },
            Some(head) => {
                let result = head.search(val);
                match result {
                    None => println!("Could not find node with value: {}", val),
                    Some(node) => println!("Found node with value: {}", node.value)
                }
                result
            }
        }
    }

    pub fn delete(mut self, val: i32) -> BST {
        match self.head {
            None => println!("BST is empty. Cannot delete {}", val),
            Some(head) => self.head = head.delete(val)
        }
        self
    }

    pub fn inorder(&self) {
        match &self.head {
            None => {},
            Some(head) => head.inorder()
        };
    }
}