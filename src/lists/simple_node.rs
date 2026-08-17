#[derive(PartialEq, Clone, Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}