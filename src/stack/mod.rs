pub struct Stack {
    stack: Vec<i32>,
    top: i32
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: Vec::new(),
            top: -1,
        }
    }

    pub fn push(&mut self, num: i32) {
        self.stack.push(num);
        self.top = num;
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
            self.top = if !self.stack.is_empty() { self.stack[self.stack.len()-1] } else { -1 }
        } else {
            println!("Stack is empty");
        }
    }

    pub fn top(&self) -> i32 {
        return self.top;
    }

    pub fn print(&self) {
        println!("Stack elements are: ");
        for i in self.stack.iter() {
            println!("{}",i);
        }
    }
}