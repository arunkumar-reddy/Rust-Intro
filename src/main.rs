mod stack;
mod graph;
mod queue;
mod linked_list;
mod bst;
mod async_await;
mod multi_threading;

use stack::Stack;
use queue::Queue;
use linked_list::LinkedList;
use bst::BST;

#[tokio::main]
async fn main() {
    hello_world();
    create_stack();
    create_queue();
    create_graph();
    create_linked_list();
    create_bst();
    run_async_tasks().await;
    multi_threading::shared_memory();
    multi_threading::message_passing();
}

fn hello_world() {
    println!("Hello, world!");
    let a = [1, 2, 3, 4];

    for (i, item) in a.iter().enumerate() {
        println!("Printing item: {} for index: {}", item, i);
    }
}

fn create_stack() {
    println!("Creating a new stack");
    let mut stack = Stack::new();
    stack.push(2);
    stack.push(3);
    stack.print();
    stack.pop();
    println!("Top is: {}", stack.top());
    stack.pop();
    stack.pop();
    println!("Top is: {}", stack.top());
    stack.push(4);
    stack.print();
}

fn create_queue() {
    println!("Creating a new queue");
    let mut queue = Queue::new();
    queue.push(2);
    queue.push(3);
    queue.print();
    queue.pop();
    println!("Front is: {}", queue.front());
    queue.pop();
    queue.pop();
    println!("Front is: {}", queue.front());
    queue.push(4);
    queue.print();   
}

fn create_graph() {
    let mut graph = graph::new(8);
    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(1,3);
    graph.add_edge(3,4);
    graph.add_edge(2,5);
    graph.add_edge(2,6);
    graph.add_edge(6,7);
    println!("DFS");
    graph.dfs();
    println!("BFS");
    graph.bfs();
}

fn create_linked_list() {
    let linked_list = LinkedList::new();
    linked_list.print_list();
}

fn create_bst() {
    let mut bst = BST::new();
    bst.insert(4);
    bst.insert(1);
    bst.insert(5);
    bst.insert(2);
    bst.insert(3);
    bst.insert(7);
    bst.inorder();
    bst = bst.delete(1);
    bst.inorder();
    bst = bst.delete(2);
    bst.inorder();
}

async fn run_async_tasks() {
    let f1 = async_await::make_api_call();
    let f2 = async_await::print_hello_world();
    tokio::join!(f1, f2);
}