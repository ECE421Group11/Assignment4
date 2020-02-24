use std::cell::RefCell;
use std::cell::Ref;


#[derive(Debug, Copy, Clone)]
pub enum Val<T>{
    V(T),
    Empty
}

#[derive(Debug, Clone)]
pub enum Node<T>{
    ValueNode{
        val: Val<T>,
        up: Box<RefCell<Node<T>>>,
        right: Box<RefCell<Node<T>>>,
    },
    TailNode
}

#[derive(Debug, Clone)]
pub struct SkipList<T>{
    //add your code here
    pub headNode: RefCell<Node<T>>,
    pub layers: RefCell<i32>,
    pub size: RefCell<usize>
}

impl<T> SkipList<T>{
    pub fn new() -> Self{
        // creates a new skip list.
        //add your code here
        let headNode:Node<T> = Node::ValueNode{
            val: Val::Empty, 
            up: Box::new(RefCell::new(Node::TailNode)), 
            right: Box::new(RefCell::new(Node::TailNode)),
        }; 

        return SkipList {
            headNode: RefCell::new(headNode),
            layers: RefCell::new(1),
            size: RefCell::new(0),
        }
    }
    
    fn len(&self) -> usize{
        // returns the number of elements at level 0 of the skip list.
        //add your code here
        
        return *self.size.borrow();
    }
    
    fn is_empty(&self) -> bool{
        // checks if the skip list is empty.
        //add your code here
        return self.len() == 0; 
    }

    pub fn insert(&self, node: Node<T>, value: T) -> Node<T>{
        return node;
    }
    
    fn push(&mut self, value: T){
        // add an element with value T to the front of the skiplist.
        //add your code here

        let s:usize = *self.size.borrow();
        let currentNode = self.headNode.borrow(); 
        let n: Node<T> = *currentNode;
        // self.headNode.replace(self.insert(currentNode, value));

        // incriment size
        // self.size.replace(self.len() + 1);
    }
    
    fn push_back(&mut self, value: T){
        // add an element with value T to the back of the skiplist.
        //add your code here
    
    }
}

fn main() {
    println!("Hello, world!");
}
