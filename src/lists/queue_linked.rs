use std::collections::VecDeque;

#[derive(PartialEq, Clone, Debug)]
pub struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            inner: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}