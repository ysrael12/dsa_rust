#[derive(PartialEq, Clone, Debug)]
pub struct Node<T>{
    pub data: T,
    pub next: Option<Box<Node<T>>>,
    pub prev: Option<Box<Node<T>>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct DoublyLinkedList<T>{
    pub head: Option<Box<Node<T>>>,
    pub tail: Option<Box<Node<T>>>,
    pub size: usize,
}

impl DoublyLinkedList<i32>{

    pub fn new() -> Self{
        DoublyLinkedList{
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push(&mut self, data: i32){
        let new_node = Box::new(Node{
            data,
            next: None,
            prev: None,
        });

        match self.tail.take(){
            Some(mut old_tail) => {
                old_tail.next = Some(new_node);
                let new_tail = old_tail.next.as_mut().unwrap();
                new_tail.prev = Some(old_tail);
                self.tail = Some(new_tail.clone());
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }
}