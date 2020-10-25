use std::collections::VecDeque;

pub struct Graph {
    adjacency_list: Vec<Vec<u32> >,
    size: u32
}

pub fn new(size: u32) -> Graph {
    let mut row = Vec::<Vec::<u32> >::new();
    for _ in 0..size {
        row.push(Vec::<u32>::new());
    }
    Graph {
        adjacency_list: row,
        size: size
    }
}

impl Graph {
    pub fn add_edge(&mut self, u: u32, v: u32) {
        if u < self.size && v < self.size {
            self.adjacency_list[u as usize].push(v);
            self.adjacency_list[v as usize].push(u);
        }
    }

    pub fn dfs(&self) {
        let mut visited = Vec::<bool>::with_capacity(self.size as usize);
        let mut stack = Vec::<u32>::new();
        for _ in 0..self.size {
            visited.push(false);
        } 
        for i in 0..self.size {
            if !visited[i as usize] {
                stack.push(i);
                while !stack.is_empty() {
                    match stack.pop() {
                        None => println!("Encountered an empty stack"),
                        Some(top) => {
                            let index = top as usize;
                            visited[index] = true;
                            println!("Visited vertex: {}", index);
                            for j in 0..self.adjacency_list[index].len() {
                                if !visited[self.adjacency_list[index][j] as usize] {
                                    stack.push(self.adjacency_list[index][j]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn bfs(&self) {
        let mut visited = Vec::<bool>::with_capacity(self.size as usize);
        let mut queue = VecDeque::new();
        for _ in 0..self.size {
            visited.push(false);
        }
        for i in 0..self.size {
            if !visited[i as usize] {
                queue.push_back(i);
                while !queue.is_empty() {
                    match queue.pop_front() {
                        None => println!("Encountered an empty queue"),
                        Some(front) => {
                            let index = front as usize;
                            visited[index] = true;
                            println!("Visited vertex: {}", index);
                            for j in 0..self.adjacency_list[index].len() {
                                if !visited[self.adjacency_list[index][j] as usize] {
                                    queue.push_back(self.adjacency_list[index][j]);
                                }
                            }
                        }
                    }
                }
            } 
        }
    }
}