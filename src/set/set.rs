#[derive(PartialEq, Clone, Debug)]
pub struct Set<T> {
    pub elements: Vec<T>,
}

impl <T: PartialEq + Clone> Set<T> {
    pub fn new() -> Self {
        Set {
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }

    pub fn remove(&mut self, element: &T) -> Option<T> {
        let index = self.elements.iter().position(|e| e == element)?;
        Some(self.elements.remove(index))
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
