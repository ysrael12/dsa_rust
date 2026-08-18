use crate::lists::simple_node::Node;

#[derive(PartialEq, Clone, Debug)] 
pub struct Stack<T> { 
    pub head: Option<Box<Node<T>>>,
    pub size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
}


// TODO -> Unit tests