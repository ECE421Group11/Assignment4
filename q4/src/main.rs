use std::cell::RefCell;
use std::cell::Ref;
use std::mem;

#[derive(Debug, Clone)]
pub enum SkipList<T>{
    //add your code here
    HeadNode(Box<SkipList<T>>, Box<SkipList<T>>), // up, right
    ValueNode(T, Box<SkipList<T>>, Box<SkipList<T>>, Box<SkipList<T>>, Box<SkipList<T>>), // value, up, right, down, left
    TailNode
}

use SkipList::*;

impl<T> SkipList<T>{
    pub fn new() -> Self{
        // creates a new skip list.
        //add your code here
        return HeadNode(Box::new(TailNode), Box::new(TailNode))
    }
    
    fn len(&self) -> usize{
        // returns the number of elements at level 0 of the skip list.
        //add your code here
        match self{
            HeadNode(up, right) => right.len(),
            ValueNode(val, up, right, down, left) => 1 + right.len(),
            TailNode => 0
        }
    }
    
    fn is_empty(&self) -> bool{
        // checks if the skip list is empty.
        //add your code here
        return self.len() == 0; 
    }

    // pub fn insert(&self, node: Node<T>, value: T) -> Node<T>{
    //     return node;
    // }
    
    fn push(&mut self, value: T){
        // add an element with value T to the front of the skiplist.
        //add your code here

        // let s:usize = *self.size.borrow();
    
        // let nodeRef: Ref<'_, Node<T>> = self.headNode.borrow(); 
        // let node: Node<T> = *nodeRef;

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
