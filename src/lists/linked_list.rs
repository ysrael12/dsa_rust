use crate::lists::simple_node::Node;

#[derive(PartialEq, Clone, Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));  // ✅ Box::new(Node::new(value))
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));  // ✅ Box::new(Node::new(value))
        
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.as_ref().unwrap().next.is_none() {
                let data = node.next.take().unwrap().data;
                self.size -= 1;
                return Some(data);
            }
            current = &mut node.next;
        }
        None
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
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