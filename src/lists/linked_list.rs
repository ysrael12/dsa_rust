use crate::simple_node::Node;



pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub size: usize,
}

impl LinkedList<i32> {

    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, data: i32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node { data, next: None }));
        } else {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node { data, next: None }));
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
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

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.data)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push(1);
        list.push(2);
        list.push(3);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(list.peek(), Some(1));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek(), Some(1));

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());
    }
}