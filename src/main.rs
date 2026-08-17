#![allow(dead_code)]
mod sort; 
mod utils;
mod trees;

use trees::binary_tree::TreeNode;

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);  
    root.insert(3);
    root.insert(7);  
    let found = root.search(&5);
    println!("Value found: {}", found); 
    let mut vector = root.to_vec();
    println!("Inorder Traversal: {:?}", vector);
}
