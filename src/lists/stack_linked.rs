use crate::simple_node::Node;

#[derive(PartialEq, Clone, Debug)] 
pub struct Stack<T>{ 
    pub head : Option<Box<Node<T>>>,
    pub tail : Option<Box<Node<T>>>,
    pub size : usize,
}

impl <T> Stack<T>{
    pub fn new(&mut self) -> Self{
        Stack{
            head: None,
            tail: None, 
            size: 0
        }
    }

    pub fn push(&mut self, value : T){
        if self.head.is_none(){
            self.head = value;
            self.head.next = None; 
        }

        let mut curr : Node<T> = self.head;
        while curr.next.is_some(){
            curr = curr.next;
        }

        curr.data = value; 

    }

     pub fn pop(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        let mut current = &mut self.head;
        let mut prev: Option<&mut Box<Node<i32>>> = None;

        while let Some(ref mut node) = current {
            if node.next.is_none() {
                let data = node.data;
                if let Some(prev_node) = prev {
                    prev_node.next = None;
                } else {
                    self.head = None;
                }
                self.size -= 1;
                return Some(data);
            }
            prev = current.as_mut();
            current = &mut node.next;
        }

        None
    }

}

// TODO -> Unit tests