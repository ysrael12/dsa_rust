#[derive(PartialEq, Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            data: value,
            next: None,
        }
    }
}

// Para lista duplamente encadeada
#[derive(PartialEq, Clone, Debug)]
pub struct DoubledNode<T> {
    pub data: T,
    pub next: Option<Box<DoubledNode<T>>>,
    pub prev: Option<Box<DoubledNode<T>>>,
}

impl<T> DoubledNode<T> {
    pub fn new(value: T) -> Self {
        DoubledNode {
            data: value,
            next: None,
            prev: None,
        }
    }
}