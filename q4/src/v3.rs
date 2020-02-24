use std::cell::RefCell;
use std::cell::Ref;
use std::mem;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub enum SkipList<T>{
    //add your code here
    HeadNode(Box<SkipList<T>>, Box<SkipList<T>>), // up, right
    ValueNode(T, Box<SkipList<T>>, Box<SkipList<T>>, Box<SkipList<T>>, Box<SkipList<T>>), // value, up, right, down, left
    TailNode
}

use SkipList::*;

impl<T: std::cmp::PartialOrd> SkipList<T>{
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
    
    fn push(&self, value: T) -> Self{
        // add an element with value T to the front of the skiplist.
        //add your code here
        match self{
            HeadNode(up, right) => right.push(value),
            ValueNode(val, up, right, down, left) => if value < *val {
                unsafe{
                    let mut newNode = ValueNode(value, Box::new(TailNode), mem::transmute_copy(self), mem::transmute_copy(left), Box::new(TailNode));
                    self.left = newNode;
                    newNode
                }
                // (*self).left = Box::new(newNode);
            }
            else{
                right.push(value)
            },
            TailNode => ValueNode(value, Box::new(TailNode), Box::new(TailNode), Box::new(TailNode), Box::new(TailNode))
        }
    }
    
    fn push_back(&mut self, value: T){
        // add an element with value T to the back of the skiplist.
        //add your code here
    
    }
}

fn main() {
    println!("Hello, world!");
}
