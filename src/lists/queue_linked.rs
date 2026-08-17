use crate::simple_node::Node;
use crate::linked_list::LinkedList;

#[derive(PartialEq, Debug, Clone)]
pub struct Queue<T>{
    head: Option<Box<Node<T>>>, 
    tail: Option<Box<Node<T>>>, 
    size: usize, 
}

/* Como os metodos vivem repetindo, vamos usar traits para aplicar polimorfismo (key idea) */
/* Desenvolvimento:  */
// Mas acho que faz mais sentido utilizar o Queue como struct especifico, pq consigo instanciar port segmentação de tipos 
// Para manter o principio de liskov do padrão Solid e inteface segmentation vamos criar uma forma especifica que lide com traits 
// trait.rs vai ter todos os tratamentos e o tipo base é LinkedList

impl <T> Queue<T>{
    pub fn new(&mut self) -> Self{ 
        Queue{ 
            head: None, 
            tail: None, 
            size: None,
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
        self.size += 1;

    }

    pub fn pop(&mut self){
        if self.head.is_none(){
            return None;
        }

        self.head.data = None; 
        self.head.next = None;
        self.size -= 1; 
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