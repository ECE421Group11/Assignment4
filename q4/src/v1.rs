use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
pub enum Value<T>{
    V(T),
    Empty
}

#[derive(Debug, Clone)]
pub enum Node<T>{
    ValueNode{
        value: Value<T>,
        up: Box<RefCell<Node<T>>>,
        right: Box<RefCell<Node<T>>>,
    },
    TailNode
}


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
            value: Value::Empty, 
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

    pub fn insert(self, mut n: Node<T>, value: T) -> Node<T>{
        match n{
            Node::ValueNode{ref value, ref up, ref right} => println!("a"),
            Node::TailNode => println!("b"),
        }
        return n;
    }
    
    fn push(&mut self, value: T){
        // add an element with value T to the front of the skiplist.
        //add your code here

        let s:usize = *self.size.borrow();
        let currentNode = *self.headNode.borrow(); 

        // (*self).insert(currentNode, value);
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
