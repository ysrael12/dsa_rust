/* Binary Tree Implementation */
#![allow(dead_code)]
type Node<T> = Option<Box<TreeNode<T>>>;

pub(crate) struct TreeNode<T> {
    pub value: T,
    pub left: Node<T>,
    pub right: Node<T>,
}


impl <T: PartialOrd> TreeNode<T> {

    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /* a balaced aproach to insert */
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
    
    pub fn inorder_traversal<'a>(&'a self, result: &mut Vec<&'a T>){

        if let Some(ref left_node) = self.left{
            left_node.inorder_traversal(result);
        }
        result.push(&self.value);
        if let Some(ref right_node) = self.right{
            right_node.inorder_traversal(result);
        }
    }

    pub fn preorder_traversal<'a>(&'a self, result: &mut Vec<&'a T>){

        result.push(&self.value);
        if let Some(ref left_node) = self.left{
            left_node.preorder_traversal(result);
        }
        if let Some(ref right_node) = self.right{
            right_node.preorder_traversal(result);
        }
    }

    pub fn postorder_traversal<'a>(&'a self, result: &mut Vec<&'a T>){

        if let Some(ref left_node) = self.left{
            left_node.postorder_traversal(result);
        }
        if let Some(ref right_node) = self.right{
            right_node.postorder_traversal(result);
        }
        result.push(&self.value);
    }

    pub fn search(&self, value: &T) -> bool {
        if *value == self.value {
            return true;
        } else if *value < self.value {
            match self.left {
                Some(ref left_node) => left_node.search(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right_node) => right_node.search(value),
                None => false,
            }
        }
    }


    pub fn height(&self) -> usize {
        let left_height = match self.left {
            Some(ref left_node) => left_node.height(),
            None => 0,
        };
        let right_height = match self.right {
            Some(ref right_node) => right_node.height(),
            None => 0,
        };
        1 + std::cmp::max(left_height, right_height)
    }

    pub fn to_vec(&self) -> Vec<&T> {
        /* [root, left, right] */
        let mut result = Vec::new();
        self.preorder_traversal(&mut result);
        result
    }

    pub fn is_bst(&self) -> bool {
        let mut result = Vec::new();
        self.inorder_traversal(&mut result);
        result.windows(2).all(|w| w[0] <= w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> TreeNode<i32> {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);
        root
    }

    #[test]
    fn inorder_is_sorted() {
        let root = tree();
        assert_eq!(root.to_vec(), vec![&3, &5, &7, &10, &12, &15, &18]);
        assert!(root.is_bst());
    }

    #[test]
    fn preorder_and_postorder() {
        let root = tree();
        let mut pre = Vec::new();
        let mut post = Vec::new();
        root.preorder_traversal(&mut pre);
        root.postorder_traversal(&mut post);
        assert_eq!(pre, vec![&10, &5, &3, &7, &15, &12, &18]);
        assert_eq!(post, vec![&3, &7, &5, &12, &18, &15, &10]);
    }

    #[test]
    fn search_finds_and_misses() {
        let root = tree();
        assert!(root.search(&7));
        assert!(root.search(&10));
        assert!(root.search(&18));
        assert!(!root.search(&1));
        assert!(!root.search(&11));
    }

    #[test]
    fn height_ok() {
        let mut root = TreeNode::new(10);
        assert_eq!(root.height(), 1);
        root.insert(5);
        root.insert(15);
        assert_eq!(root.height(), 2);
        root.insert(3);
        root.insert(7);
        assert_eq!(root.height(), 3);
    }
}
