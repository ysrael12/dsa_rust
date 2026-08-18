mod lists;

use lists::stack_linked::Stack;
use lists::queue_linked::Queue;
use lists::linked_list::LinkedList;

fn main() {
    // Teste Stack
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack pop: {:?}", stack.pop());
    println!("Stack peek: {:?}", stack.peek());
    
    // Teste Queue
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("Queue pop: {:?}", queue.pop());
    println!("Queue peek: {:?}", queue.peek());
    
    // Teste LinkedList
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("List pop: {:?}", list.pop_front());
}