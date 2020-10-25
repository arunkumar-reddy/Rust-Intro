use std::collections::VecDeque;

pub struct Queue {
    queue: VecDeque<i32>,
    front: i32
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue: VecDeque::new(),
            front: -1
        }
    }

    pub fn push(&mut self, num: i32) {
        self.queue.push_front(num);
        self.front = num;
    }

    pub fn pop(&mut self) {
        if !self.queue.is_empty() {
            self.queue.pop_front();
            self.front = if !self.queue.is_empty() { self.queue[0] } else { -1 }
        } else {
            println!("Queue is empty");
        }
    }

    pub fn front(&self) -> i32 {
        return self.front;
    }

    pub fn print(&self) {
        println!("Queue elements are: ");
        for i in self.queue.iter() {
            println!("{}",i);
        }
    }
}